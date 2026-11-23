use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    PairingRequired,
    Unsupported(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbControllerInfo {
    pub controller_name: String,
    pub vendor_id: Option<String>,
    pub product_id: Option<String>,
    pub is_thunderbolt_or_usb4: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectLinkInterface {
    pub id: String,
    pub name: String,
    pub mac_address: Option<String>,
    pub ipv4_address: Option<String>,
    pub ipv6_link_local: Option<String>,
    pub is_up: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkProperties {
    pub interface_id: String,
    pub negotiated_speed_mbps: Option<u64>,
    pub mtu: u32,
    pub is_full_duplex: bool,
}
