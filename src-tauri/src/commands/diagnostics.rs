use bytebridge_core::hardware::create_default_inspector;
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
    let inspector = create_default_inspector();
    let report = inspector
        .generate_analysis_report()
        .map_err(|e| e.to_string())?;

    Ok(DiagnosticReport {
        interface_name: report.negotiated_link.interface_name,
        negotiated_rate: report.negotiated_link.link_speed_display,
        cable_rating: report.cable.description,
        port_type: report.controller.port_type,
        measured_throughput: report
            .measured
            .raw_transport_mbps
            .map(|s| format!("{:.1} MB/s", s))
            .unwrap_or_else(|| "Pending benchmark run".to_string()),
        bottleneck_status: report.measured.primary_bottleneck,
    })
}

#[tauri::command]
pub fn export_redacted_diagnostics() -> Result<String, String> {
    let inspector = create_default_inspector();
    let diag = inspector
        .generate_redacted_diagnostics()
        .map_err(|e| e.to_string())?;

    if !diag.verify_no_sensitive_leakage() {
        return Err("Redaction policy check failed".to_string());
    }

    serde_json::to_string_pretty(&diag).map_err(|e| e.to_string())
}
