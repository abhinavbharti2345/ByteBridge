use bytebridge_core::performance::{PerformanceMode, PerformancePolicy};

#[tauri::command]
pub fn get_performance_policy(mode: String) -> Result<PerformancePolicy, String> {
    let perf_mode = match mode.to_lowercase().as_str() {
        "minimal" => PerformanceMode::Minimal,
        "maximum" => PerformanceMode::Maximum,
        _ => PerformanceMode::Adaptive,
    };
    Ok(PerformancePolicy::for_mode(perf_mode))
}

#[tauri::command]
pub fn set_performance_mode(mode: String) -> Result<PerformancePolicy, String> {
    let perf_mode = match mode.to_lowercase().as_str() {
        "minimal" => PerformanceMode::Minimal,
        "maximum" => PerformanceMode::Maximum,
        _ => PerformanceMode::Adaptive,
    };
    Ok(PerformancePolicy::for_mode(perf_mode))
}
