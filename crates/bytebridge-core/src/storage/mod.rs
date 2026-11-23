use std::path::{Path, PathBuf};
use crate::error::ByteBridgeError;

/// Storage manager handling pre-allocation, partial staging, and atomic publishing
pub struct StorageManager {
    destination_root: PathBuf,
}

impl StorageManager {
    pub fn new<P: AsRef<Path>>(destination_root: P) -> Self {
        Self {
            destination_root: destination_root.as_ref().to_path_buf(),
        }
    }

    /// Sanitize relative path to strictly forbid path traversal attacks
    pub fn sanitize_relative_path(&self, rel_path: &str) -> Result<PathBuf, ByteBridgeError> {
        let path = Path::new(rel_path);

        // Reject absolute paths, components with '..', or drive letters
        for component in path.components() {
            match component {
                std::path::Component::Normal(c) => {
                    let s = c.to_string_lossy();
                    // Reject Windows reserved device names
                    let upper = s.to_uppercase();
                    if matches!(
                        upper.as_str(),
                        "CON" | "PRN" | "AUX" | "NUL" | "COM1" | "COM2" | "COM3" | "COM4"
                            | "COM5" | "COM6" | "COM7" | "COM8" | "COM9" | "LPT1" | "LPT2"
                            | "LPT3" | "LPT4" | "LPT5" | "LPT6" | "LPT7" | "LPT8" | "LPT9"
                    ) {
                        return Err(ByteBridgeError::InvalidPath(format!(
                            "Reserved device name: {}",
                            s
                        )));
                    }
                }
                _ => {
                    return Err(ByteBridgeError::InvalidPath(format!(
                        "Invalid path component in '{}'",
                        rel_path
                    )));
                }
            }
        }

        let full_path = self.destination_root.join(path);
        Ok(full_path)
    }

    /// Get temporary staging path for an in-flight file
    pub fn staging_path(&self, file_id: u32, filename: &str) -> PathBuf {
        let partial_name = format!(".bytebridge-partial.{}.{}", file_id, filename);
        self.destination_root.join(partial_name)
    }

    /// Atomically rename the verified staging file to its final destination
    pub fn publish_file(&self, staging_path: &Path, final_path: &Path) -> Result<(), ByteBridgeError> {
        if let Some(parent) = final_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // On Windows and Linux, std::fs::rename atomically replaces within the same filesystem
        std::fs::rename(staging_path, final_path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_sanitization_rejects_traversal() {
        let sm = StorageManager::new("C:\\Transfers");
        assert!(sm.sanitize_relative_path("../evil.exe").is_err());
        assert!(sm.sanitize_relative_path("/etc/passwd").is_err());
        assert!(sm.sanitize_relative_path("C:\\Windows\\System32").is_err());
        assert!(sm.sanitize_relative_path("NUL").is_err());
        assert!(sm.sanitize_relative_path("CON").is_err());
    }

    #[test]
    fn test_path_sanitization_accepts_valid_relative_paths() {
        let sm = StorageManager::new("C:\\Transfers");
        let sanitized = sm.sanitize_relative_path("docs/report.pdf").expect("Valid relative path");
        assert!(sanitized.ends_with("docs/report.pdf"));
    }
}
