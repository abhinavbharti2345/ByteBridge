use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use crate::error::ByteBridgeError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResumeEntry {
    pub file_id: u32,
    pub relative_path: String,
    pub total_size: u64,
    pub source_mtime: Option<u64>,
    pub chunk_size: u32,
    pub highest_verified_chunk_index: u32,
    pub staging_file_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeJournal {
    pub transfer_id: Uuid,
    pub files: HashMap<u32, FileResumeEntry>,
}

impl ResumeJournal {
    pub fn new(transfer_id: Uuid) -> Self {
        Self {
            transfer_id,
            files: HashMap::new(),
        }
    }

    pub fn journal_path<P: AsRef<Path>>(dir: P, transfer_id: &Uuid) -> PathBuf {
        dir.as_ref().join(format!(".bytebridge-resume.{}.bin", transfer_id))
    }

    pub fn save_to_dir<P: AsRef<Path>>(&self, dir: P) -> Result<(), ByteBridgeError> {
        let path = Self::journal_path(dir, &self.transfer_id);
        let data = postcard::to_allocvec(self)
            .map_err(|e| ByteBridgeError::Serialization(e.to_string()))?;
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn load_from_dir<P: AsRef<Path>>(dir: P, transfer_id: &Uuid) -> Result<Option<Self>, ByteBridgeError> {
        let path = Self::journal_path(dir, transfer_id);
        if !path.exists() {
            return Ok(None);
        }
        let data = std::fs::read(&path)?;
        let journal = postcard::from_bytes(&data)
            .map_err(|e| ByteBridgeError::Serialization(e.to_string()))?;
        Ok(Some(journal))
    }

    pub fn delete_from_dir<P: AsRef<Path>>(dir: P, transfer_id: &Uuid) -> Result<(), ByteBridgeError> {
        let path = Self::journal_path(dir, transfer_id);
        if path.exists() {
            let _ = std::fs::remove_file(path);
        }
        Ok(())
    }
}
