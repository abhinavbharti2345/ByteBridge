use serde::{Deserialize, Serialize};
use bytebridge_core::integrity::HashAlgorithm;
use bytebridge_core::transfer::{Receiver, Sender};
use bytebridge_core::transport::mock::create_mock_link;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use tempfile::tempdir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferStatus {
    pub is_active: bool,
    pub file_name: String,
    pub total_bytes: u64,
    pub transferred_bytes: u64,
    pub current_mbps: f64,
    pub average_mbps: f64,
    pub eta_seconds: u64,
    pub limited_by: String,
}

#[tauri::command]
pub fn get_active_transfer() -> Result<TransferStatus, String> {
    Ok(TransferStatus {
        is_active: false,
        file_name: "".to_string(),
        total_bytes: 0,
        transferred_bytes: 0,
        current_mbps: 0.0,
        average_mbps: 0.0,
        eta_seconds: 0,
        limited_by: "None".to_string(),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockTransferResult {
    pub size_mb: usize,
    pub elapsed_seconds: f64,
    pub throughput_mbps: f64,
    pub files_transferred: usize,
    pub hash_algo: String,
}

#[tauri::command]
pub async fn start_mock_transfer(size_mb: usize) -> Result<MockTransferResult, String> {
    let source_dir = tempdir().map_err(|e| e.to_string())?;
    let dest_dir = tempdir().map_err(|e| e.to_string())?;

    let test_file_path = source_dir.path().join("simulated_payload.dat");
    let payload = vec![0x33u8; size_mb * 1024 * 1024];
    let mut f = File::create(&test_file_path).map_err(|e| e.to_string())?;
    f.write_all(&payload).map_err(|e| e.to_string())?;
    drop(f);

    let (mut sender_stream, mut receiver_stream) = create_mock_link(256 * 1024);
    let chunk_size = 1024 * 1024;
    let sender = Sender::new(chunk_size, HashAlgorithm::Blake3);
    let receiver = Receiver::new(dest_dir.path(), HashAlgorithm::Blake3);

    let files = vec![(test_file_path.clone(), "simulated_payload.dat".to_string())];

    let rx_handle = tokio::spawn(async move {
        receiver.receive_all(&mut receiver_stream).await
    });

    let start = Instant::now();
    sender
        .send_files(&mut sender_stream, &files)
        .await
        .map_err(|e| e.to_string())?;

    let received = rx_handle
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;

    let elapsed = start.elapsed().as_secs_f64();
    let throughput = (size_mb as f64) / elapsed;

    Ok(MockTransferResult {
        size_mb,
        elapsed_seconds: elapsed,
        throughput_mbps: throughput,
        files_transferred: received.len(),
        hash_algo: "BLAKE3 (Hardware Accelerated)".to_string(),
    })
}
