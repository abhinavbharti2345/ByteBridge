use bytebridge_core::hardware::create_default_inspector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStatusResponse {
    pub local_device_name: String,
    pub connection_state: String,
    pub active_interface: Option<String>,
    pub paired_devices_count: usize,
    pub is_wizard_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingSessionResponse {
    pub peer_device_name: String,
    pub verification_code: String, // 6-digit SAS
}

#[tauri::command]
pub fn get_device_status() -> Result<DeviceStatusResponse, String> {
    let inspector = create_default_inspector();
    let report = inspector.generate_analysis_report().map_err(|e| e.to_string())?;

    let is_connected = report.connection_status == bytebridge_core::hardware::ConnectionStatus::Connected;

    Ok(DeviceStatusResponse {
        local_device_name: whoami_host(),
        connection_state: if is_connected { "Connected".to_string() } else { "Disconnected".to_string() },
        active_interface: Some(report.negotiated_link.interface_name),
        paired_devices_count: if is_connected { 1 } else { 0 },
        is_wizard_required: !is_connected,
    })
}

#[tauri::command]
pub fn initiate_pairing() -> Result<PairingSessionResponse, String> {
    let fake_hash = [13u8; 32];
    let sas = bytebridge_core::security::derive_sas_code(&fake_hash);
    Ok(PairingSessionResponse {
        peer_device_name: "Direct Peer Laptop".to_string(),
        verification_code: sas,
    })
}

#[tauri::command]
pub fn confirm_pairing() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn revoke_device(device_name: String) -> Result<(), String> {
    tracing::info!("Revoked pairing for device: {}", device_name);
    Ok(())
}

fn whoami_host() -> String {
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "ByteBridge-Host".to_string())
}
