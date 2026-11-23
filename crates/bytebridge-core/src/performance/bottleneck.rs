use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BottleneckCause {
    SourceDisk,
    DestinationDisk,
    PhysicalLink,
    CpuOrCrypto,
    Unknown,
}

impl BottleneckCause {
    pub fn display_label(&self) -> &'static str {
        match self {
            BottleneckCause::SourceDisk => "Limited by: Source Disk (Read latency / throughput)",
            BottleneckCause::DestinationDisk => "Limited by: Destination Disk (Write backpressure)",
            BottleneckCause::PhysicalLink => "Limited by: Physical Link (Cable or adapter bandwidth)",
            BottleneckCause::CpuOrCrypto => "Limited by: CPU (Encryption / Hashing compute)",
            BottleneckCause::Unknown => "Balanced transfer / Link operating at nominal capacity",
        }
    }
}

/// Dynamic bottleneck analyzer with hysteresis smoothing to prevent rapid oscillation
#[derive(Debug, Clone)]
pub struct BottleneckAnalyzer {
    negotiated_link_speed_mbps: u64,
    recent_read_speeds: Vec<f64>,
    recent_write_speeds: Vec<f64>,
    recent_socket_speeds: Vec<f64>,
    window_size: usize,
}

impl BottleneckAnalyzer {
    pub fn new(negotiated_link_speed_mbps: u64) -> Self {
        Self {
            negotiated_link_speed_mbps,
            recent_read_speeds: Vec::with_capacity(10),
            recent_write_speeds: Vec::with_capacity(10),
            recent_socket_speeds: Vec::with_capacity(10),
            window_size: 5,
        }
    }

    pub fn record_sample(&mut self, read_mbps: f64, write_mbps: f64, socket_mbps: f64) {
        if self.recent_read_speeds.len() >= self.window_size {
            self.recent_read_speeds.remove(0);
        }
        if self.recent_write_speeds.len() >= self.window_size {
            self.recent_write_speeds.remove(0);
        }
        if self.recent_socket_speeds.len() >= self.window_size {
            self.recent_socket_speeds.remove(0);
        }

        self.recent_read_speeds.push(read_mbps);
        self.recent_write_speeds.push(write_mbps);
        self.recent_socket_speeds.push(socket_mbps);
    }

    /// Classify current limiting factor using smoothed averages
    pub fn classify(&self) -> BottleneckCause {
        if self.recent_socket_speeds.is_empty() {
            return BottleneckCause::Unknown;
        }

        let avg_read: f64 = self.recent_read_speeds.iter().sum::<f64>() / self.recent_read_speeds.len() as f64;
        let avg_write: f64 = self.recent_write_speeds.iter().sum::<f64>() / self.recent_write_speeds.len() as f64;
        let avg_socket: f64 = self.recent_socket_speeds.iter().sum::<f64>() / self.recent_socket_speeds.len() as f64;

        let link_max = self.negotiated_link_speed_mbps as f64;

        // If socket is near 90%+ of hardware negotiated rate, link is the constraint
        if link_max > 0.0 && avg_socket >= (link_max * 0.85) {
            return BottleneckCause::PhysicalLink;
        }

        // If write speed is lagging significantly behind read speed, destination disk is bottleneck
        if avg_write < avg_read * 0.65 {
            return BottleneckCause::DestinationDisk;
        }

        // If read speed is low while disk write capacity is higher, source disk is bottleneck
        if avg_read < avg_write * 0.65 {
            return BottleneckCause::SourceDisk;
        }

        BottleneckCause::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottleneck_classification_destination_disk() {
        let mut analyzer = BottleneckAnalyzer::new(20000); // 20 Gbps link
        // Fast reads, slow destination writes
        for _ in 0..5 {
            analyzer.record_sample(1200.0, 300.0, 300.0);
        }
        assert_eq!(analyzer.classify(), BottleneckCause::DestinationDisk);
        assert!(analyzer.classify().display_label().contains("Destination Disk"));
    }

    #[test]
    fn test_bottleneck_classification_link_saturation() {
        let mut analyzer = BottleneckAnalyzer::new(1000); // 1 Gbps link (1000 Mbps)
        for _ in 0..5 {
            analyzer.record_sample(950.0, 950.0, 920.0);
        }
        assert_eq!(analyzer.classify(), BottleneckCause::PhysicalLink);
    }
}
