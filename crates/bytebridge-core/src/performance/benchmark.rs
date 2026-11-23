use crate::error::ByteBridgeError;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::time::Instant;

/// Benchmark results across source storage, destination storage, and direct link (§6.5, §7.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport {
    pub sequential_write_mbps: f64,
    pub sequential_read_mbps: f64,
    pub link_throughput_mbps: Option<f64>,
    pub estimated_max_transfer_mbps: f64,
    pub identified_bottleneck: String,
    pub test_size_bytes: usize,
    pub duration_millis: u128,
}

/// Executes a bounded, safe storage benchmark that guarantees temporary file cleanup
pub fn run_bounded_storage_benchmark(
    target_dir: &Path,
    size_mb: usize,
) -> Result<BenchmarkReport, ByteBridgeError> {
    let size_bytes = size_mb * 1024 * 1024;
    let temp_test_file = target_dir.join(format!(".bytebridge-bench-{}.tmp", uuid::Uuid::new_v4()));

    // Guard for guaranteed cleanup on return or error
    struct TempFileGuard<'a>(&'a Path);
    impl<'a> Drop for TempFileGuard<'a> {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(self.0);
        }
    }
    let _guard = TempFileGuard(&temp_test_file);

    // 1. Sequential Write Test
    let write_buffer = vec![0xABu8; 1024 * 1024]; // 1 MiB chunk
    let write_start = Instant::now();
    {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&temp_test_file)?;

        let mut written = 0;
        while written < size_bytes {
            let to_write = std::cmp::min(write_buffer.len(), size_bytes - written);
            file.write_all(&write_buffer[..to_write])?;
            written += to_write;
        }
        file.sync_all()?;
    }
    let write_elapsed = write_start.elapsed().as_secs_f64().max(0.0001);
    let write_mbps = (size_bytes as f64 / (1024.0 * 1024.0)) / write_elapsed;

    // 2. Sequential Read Test
    let mut read_buffer = vec![0u8; 1024 * 1024];
    let read_start = Instant::now();
    {
        let mut file = File::open(&temp_test_file)?;
        let mut read_total = 0;
        while read_total < size_bytes {
            let n = file.read(&mut read_buffer)?;
            if n == 0 {
                break;
            }
            read_total += n;
        }
    }
    let read_elapsed = read_start.elapsed().as_secs_f64().max(0.0001);
    let read_mbps = (size_bytes as f64 / (1024.0 * 1024.0)) / read_elapsed;

    // Estimate bottleneck based on storage read/write disparity
    let (bottleneck, estimated_max) = if write_mbps < read_mbps * 0.7 {
        ("Destination Disk Write Throughput".to_string(), write_mbps)
    } else if read_mbps < write_mbps * 0.7 {
        ("Source Disk Read Latency".to_string(), read_mbps)
    } else {
        ("Direct Physical Link Bandwidth".to_string(), write_mbps.min(read_mbps))
    };

    Ok(BenchmarkReport {
        sequential_write_mbps: write_mbps,
        sequential_read_mbps: read_mbps,
        link_throughput_mbps: None,
        estimated_max_transfer_mbps: estimated_max,
        identified_bottleneck: bottleneck,
        test_size_bytes: size_bytes,
        duration_millis: (write_start.elapsed().as_millis()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_bounded_storage_benchmark_and_cleanup() {
        let dir = tempdir().unwrap();
        let report = run_bounded_storage_benchmark(dir.path(), 8).unwrap();
        assert!(report.sequential_write_mbps > 0.0);
        assert!(report.sequential_read_mbps > 0.0);
        assert_eq!(report.test_size_bytes, 8 * 1024 * 1024);

        // Verify temp file was cleanly removed
        let entries: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert!(entries.is_empty(), "Benchmark temp file must be cleaned up");
    }
}
