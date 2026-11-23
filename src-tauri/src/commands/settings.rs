use bytebridge_core::performance::{
    run_bounded_storage_benchmark, BenchmarkReport, PerformanceMode, PerformancePolicy,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRequest {
    pub target_folder: Option<String>,
    pub size_mb: Option<usize>,
}

#[tauri::command]
pub fn get_performance_policy(mode: Option<String>) -> Result<PerformancePolicy, String> {
    let perf_mode = match mode.as_deref() {
        Some("minimal") => PerformanceMode::Minimal,
        Some("maximum") => PerformanceMode::Maximum,
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

#[tauri::command]
pub fn run_storage_benchmark(req: BenchmarkRequest) -> Result<BenchmarkReport, String> {
    let target = req
        .target_folder
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir());

    let size_mb = req.size_mb.unwrap_or(16); // Default bounded 16 MB test
    run_bounded_storage_benchmark(&target, size_mb).map_err(|e| e.to_string())
}
