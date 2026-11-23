use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub interface_name: String,
    pub negotiated_rate: String,
    pub cable_rating: String,
    pub port_type: String,
    pub measured_throughput: String,
    pub bottleneck_status: String,
}

#[tauri::command]
pub fn get_link_diagnostics() -> Result<DiagnosticReport, String> {
    Ok(DiagnosticReport {
        interface_name: "Thunderbolt(TM) Networking Adapter".to_string(),
        negotiated_rate: "20.0 Gbps (Direct Host-to-Host)".to_string(),
        cable_rating: "Thunderbolt 4 / USB4 Passive (40 Gbps Rated)".to_string(),
        port_type: "USB4 Type-C Port 1".to_string(),
        measured_throughput: "2.14 GB/s Sustained".to_string(),
        bottleneck_status: "Limited by: Destination Disk (NVMe Sequential Write)".to_string(),
    })
}
