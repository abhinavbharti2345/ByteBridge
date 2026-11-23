use thiserror::Error;

#[derive(Error, Debug)]
pub enum ByteBridgeError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Frame too large: {0} bytes (max: {1} bytes)")]
    FrameTooLarge(usize, usize),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Hash verification failed for file {0}: expected {1}, got {2}")]
    HashMismatch(u32, String, String),

    #[error("Chunk {0} of file {1} corrupted")]
    ChunkCorrupted(u32, u32),

    #[error("Source file modified while transfer was in progress")]
    SourceFileModified,

    #[error("Destination disk full: cannot allocate {0} bytes")]
    DiskFull(u64),

    #[error("Path traversal or illegal filename rejected: {0}")]
    InvalidPath(String),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Transfer cancelled")]
    Cancelled,
}
