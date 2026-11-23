use crate::error::ByteBridgeError;
use crate::security::{derive_sas_code, NOISE_PATTERN};
use snow::{Builder, TransportState};

#[derive(Clone)]
pub struct NoiseKeypair {
    pub public: [u8; 32],
    pub private: [u8; 32],
}

pub fn generate_keypair() -> Result<NoiseKeypair, ByteBridgeError> {
    let builder = Builder::new(
        NOISE_PATTERN
            .parse()
            .map_err(|e| ByteBridgeError::AuthenticationFailed(format!("{:?}", e)))?,
    );
    let keypair = builder
        .generate_keypair()
        .map_err(|e| ByteBridgeError::AuthenticationFailed(format!("{:?}", e)))?;

    let mut public = [0u8; 32];
    let mut private = [0u8; 32];
    public.copy_from_slice(&keypair.public[..32]);
    private.copy_from_slice(&keypair.private[..32]);

    Ok(NoiseKeypair { public, private })
}

pub struct NoiseSession {
    transport: TransportState,
}

impl NoiseSession {
    pub fn new(transport: TransportState) -> Self {
        Self { transport }
    }

    /// Encrypt a plaintext frame payload in-place or returning a ciphertext buffer
    pub fn encrypt(&mut self, plaintext: &[u8]) -> Result<Vec<u8>, ByteBridgeError> {
        // Noise adds 16 bytes authentication tag
        let mut buf = vec![0u8; plaintext.len() + 16];
        let len = self
            .transport
            .write_message(plaintext, &mut buf)
            .map_err(|e| ByteBridgeError::AuthenticationFailed(format!("Encrypt failed: {:?}", e)))?;
        buf.truncate(len);
        Ok(buf)
    }

    /// Decrypt a ciphertext frame payload
    pub fn decrypt(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>, ByteBridgeError> {
        let mut buf = vec![0u8; ciphertext.len()];
        let len = self
            .transport
            .read_message(ciphertext, &mut buf)
            .map_err(|e| ByteBridgeError::AuthenticationFailed(format!("Decrypt failed: {:?}", e)))?;
        buf.truncate(len);
        Ok(buf)
    }
}

/// Perform a full 2-message Noise_IK handshake between two in-memory endpoints
pub fn perform_noise_ik_handshake(
    initiator_keys: &NoiseKeypair,
    responder_keys: &NoiseKeypair,
) -> Result<(NoiseSession, NoiseSession, String), ByteBridgeError> {
    let builder_init = Builder::new(
        NOISE_PATTERN
            .parse()
            .map_err(|e| ByteBridgeError::AuthenticationFailed(format!("{:?}", e)))?,
    );
    let builder_resp = Builder::new(
        NOISE_PATTERN
            .parse()
            .map_err(|e| ByteBridgeError::AuthenticationFailed(format!("{:?}", e)))?,
    );

    let mut init_handshake = builder_init
        .local_private_key(&initiator_keys.private)
        .remote_public_key(&responder_keys.public)
        .build_initiator()
        .map_err(|e| ByteBridgeError::AuthenticationFailed(format!("{:?}", e)))?;

    let mut resp_handshake = builder_resp
        .local_private_key(&responder_keys.private)
        .build_responder()
        .map_err(|e| ByteBridgeError::AuthenticationFailed(format!("{:?}", e)))?;

    // Message 1: Initiator -> Responder
    let mut msg1 = vec![0u8; 1024];
    let len1 = init_handshake
        .write_message(&[], &mut msg1)
        .map_err(|e| ByteBridgeError::AuthenticationFailed(format!("Msg1 write: {:?}", e)))?;
    msg1.truncate(len1);

    let mut payload1 = vec![0u8; 1024];
    resp_handshake
        .read_message(&msg1, &mut payload1)
        .map_err(|e| ByteBridgeError::AuthenticationFailed(format!("Msg1 read: {:?}", e)))?;

    // Verify initiator's static public key on responder
    let remote_static = resp_handshake
        .get_remote_static()
        .ok_or_else(|| ByteBridgeError::AuthenticationFailed("No remote static key".into()))?;
    if remote_static != initiator_keys.public {
        return Err(ByteBridgeError::AuthenticationFailed(
            "Initiator public key mismatch".into(),
        ));
    }

    // Message 2: Responder -> Initiator
    let mut msg2 = vec![0u8; 1024];
    let len2 = resp_handshake
        .write_message(&[], &mut msg2)
        .map_err(|e| ByteBridgeError::AuthenticationFailed(format!("Msg2 write: {:?}", e)))?;
    msg2.truncate(len2);

    let mut payload2 = vec![0u8; 1024];
    init_handshake
        .read_message(&msg2, &mut payload2)
        .map_err(|e| ByteBridgeError::AuthenticationFailed(format!("Msg2 read: {:?}", e)))?;

    // Derive SAS from handshake hash
    let sas_init = derive_sas_code(init_handshake.get_handshake_hash());
    let sas_resp = derive_sas_code(resp_handshake.get_handshake_hash());
    if sas_init != sas_resp {
        return Err(ByteBridgeError::AuthenticationFailed("SAS mismatch".into()));
    }

    // Transition to transport state
    let init_transport = init_handshake
        .into_transport_mode()
        .map_err(|e| ByteBridgeError::AuthenticationFailed(format!("{:?}", e)))?;
    let resp_transport = resp_handshake
        .into_transport_mode()
        .map_err(|e| ByteBridgeError::AuthenticationFailed(format!("{:?}", e)))?;

    Ok((
        NoiseSession::new(init_transport),
        NoiseSession::new(resp_transport),
        sas_init,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noise_ik_handshake_and_encryption() {
        let init_keys = generate_keypair().expect("Failed to generate initiator keys");
        let resp_keys = generate_keypair().expect("Failed to generate responder keys");

        let (mut init_session, mut resp_session, sas) =
            perform_noise_ik_handshake(&init_keys, &resp_keys)
                .expect("Handshake failed");

        assert_eq!(sas.len(), 6);

        // Test encryption: Initiator -> Responder
        let message = b"Hello encrypted direct link ByteBridge!";
        let ciphertext = init_session.encrypt(message).expect("Encryption failed");
        assert_ne!(&ciphertext[..], message);

        let decrypted = resp_session.decrypt(&ciphertext).expect("Decryption failed");
        assert_eq!(decrypted, message);

        // Test encryption: Responder -> Initiator
        let reply = b"Acknowledged with mutual authentication.";
        let reply_ciphertext = resp_session.encrypt(reply).expect("Reply encryption failed");
        let reply_decrypted = init_session.decrypt(&reply_ciphertext).expect("Reply decryption failed");
        assert_eq!(reply_decrypted, reply);
    }
}
