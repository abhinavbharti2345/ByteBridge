use crate::error::ByteBridgeError;
use crate::protocol::messages::MessageType;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub const MAX_FRAME_SIZE: usize = 16 * 1024 * 1024 + 64; // 16 MiB payload + 64 B buffer

pub struct FrameCodec;

impl FrameCodec {
    /// Read a single framed packet: [u32 length][u8 msg_type][payload]
    pub async fn read_frame<R: AsyncRead + Unpin>(
        reader: &mut R,
    ) -> Result<(MessageType, Vec<u8>), ByteBridgeError> {
        let length = reader.read_u32().await?;
        let len_usize = length as usize;

        if len_usize == 0 {
            return Err(ByteBridgeError::Protocol("Received empty frame".into()));
        }

        if len_usize > MAX_FRAME_SIZE {
            return Err(ByteBridgeError::FrameTooLarge(len_usize, MAX_FRAME_SIZE));
        }

        let msg_type_byte = reader.read_u8().await?;
        let msg_type = MessageType::try_from(msg_type_byte)?;

        let payload_len = len_usize - 1;
        let mut payload = vec![0u8; payload_len];
        reader.read_exact(&mut payload).await?;

        Ok((msg_type, payload))
    }

    /// Write a single framed packet: [u32 length][u8 msg_type][payload]
    pub async fn write_frame<W: AsyncWrite + Unpin>(
        writer: &mut W,
        msg_type: MessageType,
        payload: &[u8],
    ) -> Result<(), ByteBridgeError> {
        let len = payload.len() + 1;
        if len > MAX_FRAME_SIZE {
            return Err(ByteBridgeError::FrameTooLarge(len, MAX_FRAME_SIZE));
        }

        writer.write_u32(len as u32).await?;
        writer.write_u8(msg_type as u8).await?;
        writer.write_all(payload).await?;
        writer.flush().await?;

        Ok(())
    }

    /// Helper to encode a message with postcard and write it
    pub async fn send_message<W: AsyncWrite + Unpin, T: serde::Serialize>(
        writer: &mut W,
        msg_type: MessageType,
        message: &T,
    ) -> Result<(), ByteBridgeError> {
        let payload = postcard::to_allocvec(message)
            .map_err(|e| ByteBridgeError::Serialization(e.to_string()))?;
        Self::write_frame(writer, msg_type, &payload).await
    }

    /// Helper to decode a message payload using postcard
    pub fn decode_message<'a, T: serde::Deserialize<'a>>(
        payload: &'a [u8],
    ) -> Result<T, ByteBridgeError> {
        postcard::from_bytes(payload)
            .map_err(|e| ByteBridgeError::Serialization(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::messages::HelloMessage;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_frame_encode_decode_roundtrip() {
        let original_msg = HelloMessage {
            protocol_version: 1,
            device_name: "Laptop-Alpha".to_string(),
            os: "Windows 11".to_string(),
            hash_capabilities: 0x03,
            direct_io_supported: true,
        };

        let mut buffer = Vec::new();
        FrameCodec::send_message(&mut buffer, MessageType::Hello, &original_msg)
            .await
            .expect("Failed to send message");

        let mut cursor = Cursor::new(buffer);
        let (msg_type, payload) = FrameCodec::read_frame(&mut cursor)
            .await
            .expect("Failed to read frame");

        assert_eq!(msg_type, MessageType::Hello);
        let decoded_msg: HelloMessage =
            FrameCodec::decode_message(&payload).expect("Failed to decode payload");

        assert_eq!(decoded_msg, original_msg);
    }

    #[tokio::test]
    async fn test_oversized_frame_rejected() {
        let mut buffer = Vec::new();
        let oversized = (MAX_FRAME_SIZE + 1) as u32;
        buffer.extend_from_slice(&oversized.to_be_bytes());
        buffer.push(MessageType::Hello as u8);

        let mut cursor = Cursor::new(buffer);
        let res = FrameCodec::read_frame(&mut cursor).await;
        assert!(matches!(res, Err(ByteBridgeError::FrameTooLarge(_, _))));
    }
}
