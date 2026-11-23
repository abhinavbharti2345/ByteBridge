use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PerformanceMode {
    /// Low CPU/memory impact, conservative chunk sizes, steady background transfer
    Minimal,
    /// Balanced dynamic tuning based on storage backpressure (Default)
    #[default]
    Adaptive,
    /// Maximum throughput with sector-aligned Direct I/O and large pipelines
    Maximum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePolicy {
    pub mode: PerformanceMode,
    pub chunk_size: usize,
    pub pipeline_depth: usize,
    pub use_direct_io: bool,
}

impl PerformancePolicy {
    pub fn for_mode(mode: PerformanceMode) -> Self {
        match mode {
            PerformanceMode::Minimal => Self {
                mode,
                chunk_size: 512 * 1024, // 512 KiB
                pipeline_depth: 2,
                use_direct_io: false,
            },
            PerformanceMode::Adaptive => Self {
                mode,
                chunk_size: 1024 * 1024, // 1 MiB baseline
                pipeline_depth: 4,
                use_direct_io: false,
            },
            PerformanceMode::Maximum => Self {
                mode,
                chunk_size: 8 * 1024 * 1024, // 8 MiB
                pipeline_depth: 12,
                use_direct_io: true,
            },
        }
    }
}
