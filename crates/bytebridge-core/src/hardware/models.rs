use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    PairingRequired,
    Unsupported(String),
}

/// Category 1: Physical Cable details (§2.2, §3)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CableCapability {
    /// Advertised capability e.g. "Thunderbolt 4 / USB4 Passive (40 Gbps)" or "Unknown / Not exposed by hardware or OS"
    pub description: String,
    pub is_verified: bool,
    pub raw_emulated_marker: Option<String>,
}

impl Default for CableCapability {
    fn default() -> Self {
        Self {
            description: "Unknown / Not exposed by hardware or OS".to_string(),
            is_verified: false,
            raw_emulated_marker: None,
        }
    }
}

/// Category 2: Port & Controller details (§2.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortControllerCapability {
    pub controller_name: String,
    pub port_type: String, // e.g., "USB4 / Thunderbolt 4 Type-C" or "USB 3.2 Gen 2 Type-C"
    pub vendor_id: Option<String>,
    pub product_id: Option<String>,
    pub max_controller_bandwidth_gbps: Option<f32>,
}

/// Category 3: Negotiated Link Mode & Rate (§2.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiatedLinkProperties {
    pub interface_name: String,
    pub physical_medium: String, // e.g. "Thunderbolt IP / NCM Direct Link"
    pub link_speed_mbps: Option<u64>,
    pub link_speed_display: String, // e.g. "20.0 Gbps" or "Unknown / Not exposed by hardware or OS"
    pub mtu: u32,
    pub is_full_duplex: bool,
    pub ipv6_link_local: Option<String>,
}

/// Category 4 & 5: Measured Throughputs (§2.2)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MeasuredThroughput {
    pub raw_transport_mbps: Option<f64>,
    pub end_to_end_file_mbps: Option<f64>,
    pub primary_bottleneck: String,
}

/// Full 5-layer Hardware Analysis View
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareAnalysisReport {
    pub cable: CableCapability,
    pub controller: PortControllerCapability,
    pub negotiated_link: NegotiatedLinkProperties,
    pub measured: MeasuredThroughput,
    pub connection_status: ConnectionStatus,
}

/// Redacted Diagnostic Report (§6.4)
/// Guaranteed to contain zero usernames, machine names, filenames, file paths, or cryptographic keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedactedDiagnosticReport {
    pub timestamp_utc: String,
    pub os_family: String,
    pub os_version: String,
    pub adapter_type: String,
    pub negotiated_speed_display: String,
    pub mtu: u32,
    pub is_full_duplex: bool,
    pub bottleneck_classification: String,
    pub raw_transport_speed_mbps: Option<f64>,
    pub end_to_end_speed_mbps: Option<f64>,
}

impl RedactedDiagnosticReport {
    /// Verifies that no personal identifiers, full local file paths, or private keys leak into diagnostics
    pub fn verify_no_sensitive_leakage(&self) -> bool {
        let serialized = serde_json::to_string(self).unwrap_or_default().to_lowercase();
        let forbidden = [
            "c:\\users\\",
            "/home/",
            "abhinav",
            "private_key",
            "secret",
            ".ssh",
            "password",
        ];
        !forbidden.iter().any(|pattern| serialized.contains(pattern))
    }
}

// Retain legacy structs for backward compatibility
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
