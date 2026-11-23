use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HashAlgorithm {
    Sha256 = 1,
    Blake3 = 2,
}

pub const HASH_CAP_SHA256: u8 = 0x01;
pub const HASH_CAP_BLAKE3: u8 = 0x02;

/// Streaming incremental hasher supporting both SHA-256 and BLAKE3
pub enum Hasher {
    Sha256(Sha256),
    Blake3(blake3::Hasher),
}

impl Hasher {
    pub fn new(algo: HashAlgorithm) -> Self {
        match algo {
            HashAlgorithm::Sha256 => Hasher::Sha256(Sha256::new()),
            HashAlgorithm::Blake3 => Hasher::Blake3(blake3::Hasher::new()),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        match self {
            Hasher::Sha256(h) => h.update(data),
            Hasher::Blake3(h) => {
                h.update(data);
            }
        }
    }

    pub fn finalize(self) -> [u8; 32] {
        match self {
            Hasher::Sha256(h) => h.finalize().into(),
            Hasher::Blake3(h) => *h.finalize().as_bytes(),
        }
    }
}

/// Compute a one-shot 32-byte hash over a buffer
pub fn compute_hash(algo: HashAlgorithm, data: &[u8]) -> [u8; 32] {
    let mut hasher = Hasher::new(algo);
    hasher.update(data);
    hasher.finalize()
}

/// Format 32-byte hash as lowercase hex string
pub fn to_hex(hash: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for b in hash {
        use std::fmt::Write;
        let _ = write!(s, "{:02x}", b);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hashing() {
        let data = b"bytebridge direct transfer test data";
        let hash = compute_hash(HashAlgorithm::Sha256, data);
        assert_eq!(hash.len(), 32);
        let hex = to_hex(&hash);
        assert_eq!(hex.len(), 64);
    }

    #[test]
    fn test_blake3_hashing() {
        let data = b"bytebridge direct transfer test data";
        let hash = compute_hash(HashAlgorithm::Blake3, data);
        assert_eq!(hash.len(), 32);
        let hex = to_hex(&hash);
        assert_eq!(hex.len(), 64);
    }

    #[test]
    fn test_incremental_hasher() {
        let chunk1 = b"bytebridge ";
        let chunk2 = b"direct transfer";
        
        let mut h = Hasher::new(HashAlgorithm::Blake3);
        h.update(chunk1);
        h.update(chunk2);
        let hash1 = h.finalize();

        let hash2 = compute_hash(HashAlgorithm::Blake3, b"bytebridge direct transfer");
        assert_eq!(hash1, hash2);
    }
}
