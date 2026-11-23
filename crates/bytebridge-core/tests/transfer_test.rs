use bytebridge_core::integrity::HashAlgorithm;
use bytebridge_core::transfer::{Receiver, Sender};
use bytebridge_core::transport::mock::create_mock_link;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[tokio::test]
async fn test_end_to_end_mock_transfer() {
    let source_dir = tempdir().expect("Failed to create source tempdir");
    let dest_dir = tempdir().expect("Failed to create dest tempdir");

    // Create a 500 KB test file in source
    let file1_path = source_dir.path().join("document.pdf");
    let mut file1 = File::create(&file1_path).expect("Failed to create file1");
    let test_data = vec![0xABu8; 500 * 1024];
    file1.write_all(&test_data).expect("Failed to write file1");
    drop(file1);

    // Create an in-process duplex transport pair simulating a direct link
    let (mut sender_stream, mut receiver_stream) = create_mock_link(64 * 1024);

    let sender = Sender::new(64 * 1024, HashAlgorithm::Blake3); // 64 KiB chunks
    let receiver = Receiver::new(dest_dir.path(), HashAlgorithm::Blake3);

    let files_to_send = vec![(file1_path.clone(), "documents/report.pdf".to_string())];

    // Spawn receiver task
    let receiver_handle = tokio::spawn(async move {
        receiver.receive_all(&mut receiver_stream).await
    });

    // Run sender
    sender
        .send_files(&mut sender_stream, &files_to_send)
        .await
        .expect("Sender failed");

    let received_files = receiver_handle
        .await
        .expect("Receiver panicked")
        .expect("Receiver failed");

    assert_eq!(received_files.len(), 1);
    let output_file_path = &received_files[0];

    assert!(output_file_path.exists());
    let output_data = std::fs::read(output_file_path).expect("Failed to read received file");
    assert_eq!(output_data.len(), test_data.len());
    assert_eq!(output_data, test_data);
}

#[tokio::test]
async fn test_multi_file_and_nested_folder_transfer() {
    let source_dir = tempdir().expect("Failed to create source tempdir");
    let dest_dir = tempdir().expect("Failed to create dest tempdir");

    // Create 3 files in source: one root, two in nested directory
    let f1_path = source_dir.path().join("readme.txt");
    std::fs::write(&f1_path, b"ByteBridge transfer test root file").unwrap();

    let nested_dir = source_dir.path().join("subfolder").join("deep");
    std::fs::create_dir_all(&nested_dir).unwrap();
    let f2_path = nested_dir.join("data.bin");
    let binary_data = vec![0xEFu8; 128 * 1024];
    std::fs::write(&f2_path, &binary_data).unwrap();

    let (mut sender_stream, mut receiver_stream) = create_mock_link(64 * 1024);

    let sender = Sender::new(32 * 1024, HashAlgorithm::Sha256);
    let receiver = Receiver::new(dest_dir.path(), HashAlgorithm::Sha256);

    let files_to_send = vec![
        (f1_path.clone(), "readme.txt".to_string()),
        (f2_path.clone(), "subfolder/deep/data.bin".to_string()),
    ];

    let receiver_handle = tokio::spawn(async move {
        receiver.receive_all(&mut receiver_stream).await
    });

    sender
        .send_files(&mut sender_stream, &files_to_send)
        .await
        .expect("Sender failed");

    let received_files = receiver_handle
        .await
        .expect("Receiver panicked")
        .expect("Receiver failed");

    assert_eq!(received_files.len(), 2);

    let received_f1 = dest_dir.path().join("readme.txt");
    assert_eq!(std::fs::read(received_f1).unwrap(), b"ByteBridge transfer test root file");

    let received_f2 = dest_dir.path().join("subfolder").join("deep").join("data.bin");
    assert_eq!(std::fs::read(received_f2).unwrap(), binary_data);
}

