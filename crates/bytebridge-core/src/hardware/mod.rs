pub mod models;

pub use models::{DirectLinkInterface, UsbControllerInfo, LinkProperties, ConnectionStatus};
use crate::error::ByteBridgeError;

/// Trait abstracting OS-specific hardware and direct-link interface inspection
pub trait HardwareInspector: Send + Sync {
    /// Enumerate USB/Thunderbolt host controllers
    fn inspect_controllers(&self) -> Result<Vec<UsbControllerInfo>, ByteBridgeError>;

    /// Enumerate candidate direct-link network interfaces
    fn inspect_interfaces(&self) -> Result<Vec<DirectLinkInterface>, ByteBridgeError>;

    /// Query negotiated link speed and duplex status for an interface
    fn inspect_link_properties(&self, iface_id: &str) -> Result<LinkProperties, ByteBridgeError>;
}
