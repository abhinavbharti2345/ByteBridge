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
    Ok(DeviceStatusResponse {
        local_device_name: whoami_host(),
        connection_state: "Connected".to_string(),
        active_interface: Some("Thunderbolt(TM) Networking (Direct Link)".to_string()),
        paired_devices_count: 1,
        is_wizard_required: false,
    })
}

#[tauri::command]
pub fn initiate_pairing() -> Result<PairingSessionResponse, String> {
    // Generate pairing verification code
    let fake_hash = [13u8; 32];
    let sas = bytebridge_core::security::derive_sas_code(&fake_hash);
    Ok(PairingSessionResponse {
        peer_device_name: "Target Laptop".to_string(),
        verification_code: sas,
    })
}

#[tauri::command]
pub fn confirm_pairing() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn revoke_device(device_name: String) -> Result<(), String> {
    println!("Revoked pairing for device: {}", device_name);
    Ok(())
}

fn whoami_host() -> String {
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "ByteBridge-Host".to_string())
}
