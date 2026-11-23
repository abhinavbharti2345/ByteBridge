use bytebridge_core::device::PairedDevice;
use bytebridge_core::security::trust_store::{MemoryTrustStore, TrustStore};
use bytebridge_core::storage::StorageManager;
use tempfile::tempdir;

#[test]
fn test_threat_unpaired_peer_rejected() {
    let trust_store = MemoryTrustStore::new();

    let valid_peer = [0x11u8; 32];
    let attacker_peer = [0x99u8; 32];

    trust_store
        .add_paired_device(PairedDevice {
            device_name: "laptop-a".to_string(),
            os: "Windows 11".to_string(),
            public_key: valid_peer,
            paired_timestamp: 1700000000,
        })
        .unwrap();

    assert!(trust_store.is_paired(&valid_peer));
    assert!(
        !trust_store.is_paired(&attacker_peer),
        "Unpaired peer must strictly be rejected"
    );
}

#[test]
fn test_threat_pairing_revocation_blocks_reconnection() {
    let trust_store = MemoryTrustStore::new();

    let peer_key = [0x42u8; 32];
    trust_store
        .add_paired_device(PairedDevice {
            device_name: "revocable-laptop".to_string(),
            os: "Linux Ubuntu 24.04".to_string(),
            public_key: peer_key,
            paired_timestamp: 1700000000,
        })
        .unwrap();
    assert!(trust_store.is_paired(&peer_key));

    // Execute revocation
    trust_store.revoke_device(&peer_key).unwrap();
    assert!(
        !trust_store.is_paired(&peer_key),
        "Revoked peer must immediately be rejected from trust store"
    );
}

#[test]
fn test_threat_path_traversal_fuzzing() {
    let dir = tempdir().unwrap();
    let storage = StorageManager::new(dir.path());

    let malicious_paths = [
        "../../etc/passwd",
        r"..\..\Windows\System32\cmd.exe",
        "/absolute/root/file",
        r"C:\Users\Target\secret.txt",
        r"D:\data\payload.iso",
        "nested/../../escape.txt",
        "CON",
        "NUL",
        "COM1",
        "AUX",
        "PRN",
    ];

    for path in malicious_paths {
        let result = storage.sanitize_relative_path(path);
        assert!(
            result.is_err(),
            "Malicious or reserved path '{}' must be rejected, but was accepted as: {:?}",
            path,
            result
        );
    }
}

#[test]
fn test_threat_valid_nested_paths_accepted() {
    let dir = tempdir().unwrap();
    let storage = StorageManager::new(dir.path());

    let safe_paths = [
        "documents/report.pdf",
        "source/code/main.rs",
        "photos/2026/vacation/sunset.jpg",
        "data.csv",
        "my_folder/sub_folder/deep_folder/archive.tar",
    ];

    for path in safe_paths {
        let sanitized = storage.sanitize_relative_path(path).unwrap();
        assert!(!sanitized.to_string_lossy().contains(".."));
    }
}

#[test]
fn test_threat_atomic_publishing_integrity() {
    let dir = tempdir().unwrap();
    let storage = StorageManager::new(dir.path());

    let test_file = "safepayload.bin";
    let data = b"Secure verified payload content over direct link";

    // Create partial staging file
    let staging_path = storage.staging_path(1, test_file);
    std::fs::write(&staging_path, data).unwrap();
    assert!(staging_path.exists());

    let final_path = storage.sanitize_relative_path(test_file).unwrap();

    // Atomically publish
    storage.publish_file(&staging_path, &final_path).unwrap();

    assert!(final_path.exists());
    assert!(!staging_path.exists(), "Partial staging file must be atomically replaced");

    let content = std::fs::read(final_path).unwrap();
    assert_eq!(content, data);
}
