use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum MessageType {
    Hello = 0x01,
    Manifest = 0x02,
    ManifestDecision = 0x03,
    FileStart = 0x04,
    Chunk = 0x05,
    ChunkAck = 0x06,
    FileComplete = 0x07,
    FileResult = 0x08,
    Resume = 0x09,
    Cancel = 0x0A,
    Error = 0x0B,
}

impl TryFrom<u8> for MessageType {
    type Error = crate::error::ByteBridgeError;

    fn try_from(val: u8) -> Result<Self, crate::error::ByteBridgeError> {
        match val {
            0x01 => Ok(MessageType::Hello),
            0x02 => Ok(MessageType::Manifest),
            0x03 => Ok(MessageType::ManifestDecision),
            0x04 => Ok(MessageType::FileStart),
            0x05 => Ok(MessageType::Chunk),
            0x06 => Ok(MessageType::ChunkAck),
            0x07 => Ok(MessageType::FileComplete),
            0x08 => Ok(MessageType::FileResult),
            0x09 => Ok(MessageType::Resume),
            0x0A => Ok(MessageType::Cancel),
            0x0B => Ok(MessageType::Error),
            other => Err(crate::error::ByteBridgeError::Protocol(format!(
                "Unknown message type byte: 0x{:02x}",
                other
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelloMessage {
    pub protocol_version: u16,
    pub device_name: String,
    pub os: String,
    pub hash_capabilities: u8,
    pub direct_io_supported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestEntry {
    pub file_id: u32,
    pub relative_path: String,
    pub total_size: u64,
    pub modified_time: Option<u64>,
    pub is_symlink: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestMessage {
    pub transfer_id: Uuid,
    pub entries: Vec<ManifestEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestDecisionMessage {
    pub transfer_id: Uuid,
    pub accepted_file_ids: Vec<u32>,
    pub rejected_file_ids: Vec<(u32, String)>,
    pub negotiated_hash_algo: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileStartMessage {
    pub file_id: u32,
    pub chunk_size: u32,
    pub total_chunks: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkMessage {
    pub file_id: u32,
    pub chunk_index: u32,
    pub data: Vec<u8>,
    pub chunk_hash: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkAckMessage {
    pub file_id: u32,
    pub highest_verified_chunk: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileCompleteMessage {
    pub file_id: u32,
    pub whole_file_hash: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileResultMessage {
    pub file_id: u32,
    pub success: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeMessage {
    pub file_id: u32,
    pub resume_chunk_index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelMessage {
    pub file_id: Option<u32>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: String,
}
