use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use tokio::io::{AsyncRead, AsyncWrite};

use crate::error::ByteBridgeError;
use crate::integrity::{compute_hash, to_hex, HashAlgorithm, Hasher};
use crate::protocol::frames::FrameCodec;
use crate::protocol::messages::{
    ChunkAckMessage, ChunkMessage, FileCompleteMessage, FileResultMessage, FileStartMessage,
    ManifestDecisionMessage, ManifestMessage, MessageType,
};
use crate::storage::StorageManager;
use crate::transfer::journal::{FileResumeEntry, ResumeJournal};

pub struct Receiver {
    destination_root: PathBuf,
    storage: StorageManager,
    hash_algo: HashAlgorithm,
}

impl Receiver {
    pub fn new<P: AsRef<Path>>(destination_root: P, hash_algo: HashAlgorithm) -> Self {
        let dest = destination_root.as_ref().to_path_buf();
        Self {
            storage: StorageManager::new(&dest),
            destination_root: dest,
            hash_algo,
        }
    }

    /// Process incoming manifest and execute complete file reception over the stream
    pub async fn receive_all<S: AsyncRead + AsyncWrite + Unpin>(
        &self,
        stream: &mut S,
    ) -> Result<Vec<PathBuf>, ByteBridgeError> {
        // 1. Read Manifest
        let (msg_type, payload) = FrameCodec::read_frame(stream).await?;
        if msg_type != MessageType::Manifest {
            return Err(ByteBridgeError::Protocol(format!(
                "Expected Manifest, got {:?}",
                msg_type
            )));
        }
        let manifest: ManifestMessage = FrameCodec::decode_message(&payload)?;

        let mut accepted_file_ids = Vec::new();
        let mut rejected_file_ids = Vec::new();
        let mut final_paths = Vec::new();

        let mut journal = ResumeJournal::new(manifest.transfer_id);

        for entry in &manifest.entries {
            match self.storage.sanitize_relative_path(&entry.relative_path) {
                Ok(path) => {
                    accepted_file_ids.push(entry.file_id);
                    final_paths.push((entry.file_id, entry.relative_path.clone(), path));
                }
                Err(err) => {
                    rejected_file_ids.push((entry.file_id, err.to_string()));
                }
            }
        }

        // 2. Send ManifestDecision
        let decision = ManifestDecisionMessage {
            transfer_id: manifest.transfer_id,
            accepted_file_ids: accepted_file_ids.clone(),
            rejected_file_ids,
            negotiated_hash_algo: self.hash_algo as u8,
        };
        FrameCodec::send_message(stream, MessageType::ManifestDecision, &decision).await?;

        let mut completed_files = Vec::new();

        // 3. Receive each accepted file
        for (file_id, rel_path, final_path) in final_paths {
            // Read FileStart
            let (msg_type, payload) = FrameCodec::read_frame(stream).await?;
            if msg_type != MessageType::FileStart {
                return Err(ByteBridgeError::Protocol(format!(
                    "Expected FileStart, got {:?}",
                    msg_type
                )));
            }
            let file_start: FileStartMessage = FrameCodec::decode_message(&payload)?;

            let filename = final_path.file_name().unwrap().to_string_lossy().to_string();
            let staging_path = self.storage.staging_path(file_id, &filename);

            // Open/create staging file
            let mut file = OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open(&staging_path)?;

            // Initialize journal entry
            let resume_entry = FileResumeEntry {
                file_id,
                relative_path: rel_path.clone(),
                total_size: 0,
                source_mtime: None,
                chunk_size: file_start.chunk_size,
                highest_verified_chunk_index: 0,
                staging_file_name: staging_path.file_name().unwrap().to_string_lossy().to_string(),
            };
            journal.files.insert(file_id, resume_entry);

            let mut whole_hasher = Hasher::new(self.hash_algo);

            // Read Chunks
            for expected_index in 0..file_start.total_chunks {
                let (msg_type, payload) = FrameCodec::read_frame(stream).await?;
                if msg_type != MessageType::Chunk {
                    return Err(ByteBridgeError::Protocol(format!(
                        "Expected Chunk, got {:?}",
                        msg_type
                    )));
                }
                let chunk_msg: ChunkMessage = FrameCodec::decode_message(&payload)?;

                if chunk_msg.chunk_index != expected_index {
                    return Err(ByteBridgeError::Protocol(format!(
                        "Chunk index out of order: expected {}, got {}",
                        expected_index, chunk_msg.chunk_index
                    )));
                }

                // Verify chunk hash
                let calculated_chunk_hash = compute_hash(self.hash_algo, &chunk_msg.data);
                if calculated_chunk_hash != chunk_msg.chunk_hash {
                    return Err(ByteBridgeError::ChunkCorrupted(file_id, expected_index));
                }

                // Write to file at proper offset
                let offset = (chunk_msg.chunk_index as u64) * (file_start.chunk_size as u64);
                file.seek(SeekFrom::Start(offset))?;
                file.write_all(&chunk_msg.data)?;
                whole_hasher.update(&chunk_msg.data);

                // Send ChunkAck
                let ack = ChunkAckMessage {
                    file_id,
                    highest_verified_chunk: chunk_msg.chunk_index,
                };
                FrameCodec::send_message(stream, MessageType::ChunkAck, &ack).await?;
            }

            file.flush()?;

            // Read FileComplete
            let (msg_type, payload) = FrameCodec::read_frame(stream).await?;
            if msg_type != MessageType::FileComplete {
                return Err(ByteBridgeError::Protocol(format!(
                    "Expected FileComplete, got {:?}",
                    msg_type
                )));
            }
            let complete_msg: FileCompleteMessage = FrameCodec::decode_message(&payload)?;

            let final_calculated_hash = whole_hasher.finalize();
            if final_calculated_hash != complete_msg.whole_file_hash {
                let expected = to_hex(&complete_msg.whole_file_hash);
                let got = to_hex(&final_calculated_hash);

                let result = FileResultMessage {
                    file_id,
                    success: false,
                    error_message: Some(format!("Hash mismatch: expected {}, got {}", expected, got)),
                };
                FrameCodec::send_message(stream, MessageType::FileResult, &result).await?;

                return Err(ByteBridgeError::HashMismatch(file_id, expected, got));
            }

            // Atomically publish
            drop(file);
            self.storage.publish_file(&staging_path, &final_path)?;

            // Send FileResult (Success)
            let result = FileResultMessage {
                file_id,
                success: true,
                error_message: None,
            };
            FrameCodec::send_message(stream, MessageType::FileResult, &result).await?;

            completed_files.push(final_path);
        }

        // Clean up resume journal
        let _ = ResumeJournal::delete_from_dir(&self.destination_root, &manifest.transfer_id);

        Ok(completed_files)
    }
}
