pub mod session;
pub mod trust_store;

pub use session::{NoiseSession, NoiseKeypair, generate_keypair};
pub use trust_store::{TrustStore, MemoryTrustStore};

use sha2::{Digest, Sha256};

pub const NOISE_PATTERN: &str = "Noise_IK_25519_ChaChaPoly_BLAKE2s";

/// Derives a 6-digit Short Authentication String (SAS) from the Noise handshake hash
pub fn derive_sas_code(handshake_hash: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"ByteBridge-SAS-v1");
    hasher.update(handshake_hash);
    let result = hasher.finalize();

    // Use first 4 bytes as a big-endian u32 modulo 1,000,000 for 6 digits
    let num = u32::from_be_bytes([result[0], result[1], result[2], result[3]]) % 1_000_000;
    format!("{:06}", num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sas_code_derivation() {
        let fake_handshake_hash = [42u8; 32];
        let sas1 = derive_sas_code(&fake_handshake_hash);
        let sas2 = derive_sas_code(&fake_handshake_hash);
        assert_eq!(sas1, sas2);
        assert_eq!(sas1.len(), 6);
    }
}
