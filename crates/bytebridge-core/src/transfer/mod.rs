pub mod journal;
pub mod receiver;
pub mod sender;

pub use journal::{FileResumeEntry, ResumeJournal};
pub use receiver::Receiver;
pub use sender::Sender;

pub mod progress {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TransferProgress {
        pub file_id: u32,
        pub bytes_transferred: u64,
        pub total_bytes: u64,
        pub current_speed_mbps: f64,
        pub average_speed_mbps: f64,
        pub eta_seconds: u64,
        pub limited_by: String,
    }
}
