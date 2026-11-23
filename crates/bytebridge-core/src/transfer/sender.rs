use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tokio::io::{AsyncRead, AsyncWrite};
use uuid::Uuid;

use crate::error::ByteBridgeError;
use crate::integrity::{compute_hash, HashAlgorithm, Hasher};
use crate::protocol::frames::FrameCodec;
use crate::protocol::messages::{
    ChunkAckMessage, ChunkMessage, FileCompleteMessage, FileResultMessage, FileStartMessage,
    ManifestDecisionMessage, ManifestEntry, ManifestMessage, MessageType,
};

pub struct Sender {
    chunk_size: u32,
    hash_algo: HashAlgorithm,
}

impl Sender {
    pub fn new(chunk_size: u32, hash_algo: HashAlgorithm) -> Self {
        Self {
            chunk_size,
            hash_algo,
        }
    }

    /// Send a set of local files across the streaming transport
    pub async fn send_files<S: AsyncRead + AsyncWrite + Unpin>(
        &self,
        stream: &mut S,
        files: &[(PathBuf, String)], // (Absolute Path, Relative Path)
    ) -> Result<(), ByteBridgeError> {
        let transfer_id = Uuid::new_v4();

        // 1. Build & Send Manifest
        let mut manifest_entries = Vec::new();
        for (idx, (abs_path, rel_path)) in files.iter().enumerate() {
            let metadata = std::fs::metadata(abs_path)?;
            let modified_time = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs());

            manifest_entries.push(ManifestEntry {
                file_id: (idx + 1) as u32,
                relative_path: rel_path.clone(),
                total_size: metadata.len(),
                modified_time,
                is_symlink: metadata.file_type().is_symlink(),
            });
        }

        let manifest = ManifestMessage {
            transfer_id,
            entries: manifest_entries,
        };
        FrameCodec::send_message(stream, MessageType::Manifest, &manifest).await?;

        // 2. Read ManifestDecision
        let (msg_type, payload) = FrameCodec::read_frame(stream).await?;
        if msg_type != MessageType::ManifestDecision {
            return Err(ByteBridgeError::Protocol(format!(
                "Expected ManifestDecision, got {:?}",
                msg_type
            )));
        }
        let decision: ManifestDecisionMessage = FrameCodec::decode_message(&payload)?;

        // 3. Send each accepted file
        for (idx, (abs_path, _)) in files.iter().enumerate() {
            let file_id = (idx + 1) as u32;
            if !decision.accepted_file_ids.contains(&file_id) {
                continue;
            }

            let mut file = File::open(abs_path)?;
            let total_size = file.metadata()?.len();
            let total_chunks = if total_size == 0 {
                1
            } else {
                ((total_size + (self.chunk_size as u64) - 1) / (self.chunk_size as u64)) as u32
            };

            // Send FileStart
            let file_start = FileStartMessage {
                file_id,
                chunk_size: self.chunk_size,
                total_chunks,
            };
            FrameCodec::send_message(stream, MessageType::FileStart, &file_start).await?;

            let mut whole_hasher = Hasher::new(self.hash_algo);
            let mut buffer = vec![0u8; self.chunk_size as usize];

            for chunk_index in 0..total_chunks {
                let bytes_read = file.read(&mut buffer)?;
                let chunk_data = &buffer[..bytes_read];

                let chunk_hash = compute_hash(self.hash_algo, chunk_data);
                whole_hasher.update(chunk_data);

                let chunk_msg = ChunkMessage {
                    file_id,
                    chunk_index,
                    data: chunk_data.to_vec(),
                    chunk_hash,
                };
                FrameCodec::send_message(stream, MessageType::Chunk, &chunk_msg).await?;

                // Read ChunkAck
                let (ack_type, ack_payload) = FrameCodec::read_frame(stream).await?;
                if ack_type != MessageType::ChunkAck {
                    return Err(ByteBridgeError::Protocol(format!(
                        "Expected ChunkAck, got {:?}",
                        ack_type
                    )));
                }
                let _ack: ChunkAckMessage = FrameCodec::decode_message(&ack_payload)?;
            }

            // Send FileComplete
            let whole_file_hash = whole_hasher.finalize();
            let complete_msg = FileCompleteMessage {
                file_id,
                whole_file_hash,
            };
            FrameCodec::send_message(stream, MessageType::FileComplete, &complete_msg).await?;

            // Read FileResult
            let (res_type, res_payload) = FrameCodec::read_frame(stream).await?;
            if res_type != MessageType::FileResult {
                return Err(ByteBridgeError::Protocol(format!(
                    "Expected FileResult, got {:?}",
                    res_type
                )));
            }
            let file_result: FileResultMessage = FrameCodec::decode_message(&res_payload)?;

            if !file_result.success {
                return Err(ByteBridgeError::Protocol(format!(
                    "File {} rejected by receiver: {:?}",
                    file_id, file_result.error_message
                )));
            }
        }

        Ok(())
    }
}
