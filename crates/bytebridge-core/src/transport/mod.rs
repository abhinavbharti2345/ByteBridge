//! Transport abstractions for direct-link TCP and in-process mock simulation

use tokio::io::{AsyncRead, AsyncWrite};

/// Trait abstracting bidirectional duplex byte streams
pub trait ByteBridgeStream: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> ByteBridgeStream for T {}

pub mod mock {
    use tokio::io::DuplexStream;

    /// Create an in-process duplex pair simulating a physical direct link
    pub fn create_mock_link(buffer_size: usize) -> (DuplexStream, DuplexStream) {
        tokio::io::duplex(buffer_size)
    }
}
