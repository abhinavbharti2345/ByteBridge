pub mod commands;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::devices::get_device_status,
            commands::devices::initiate_pairing,
            commands::devices::confirm_pairing,
            commands::devices::revoke_device,
            commands::transfers::get_active_transfer,
            commands::transfers::start_mock_transfer,
            commands::settings::get_performance_policy,
            commands::settings::set_performance_mode,
            commands::settings::run_storage_benchmark,
            commands::diagnostics::get_link_diagnostics,
            commands::diagnostics::export_redacted_diagnostics,
        ])
        .run(tauri::generate_context!())
        .expect("error while running ByteBridge desktop application");
}
