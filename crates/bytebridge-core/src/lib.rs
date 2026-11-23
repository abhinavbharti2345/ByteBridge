//! # ByteBridge Core Engine
//!
//! `bytebridge-core` is the standalone, cross-platform engine powering direct
//! laptop-to-laptop file transfers over verified USB4, Thunderbolt, and host-to-host links.

pub mod protocol;
pub mod integrity;
pub mod storage;
pub mod security;
pub mod transport;
pub mod transfer;
pub mod hardware;
pub mod device;
pub mod performance;
pub mod error;

pub use error::ByteBridgeError;
pub use performance::{PerformanceMode, PerformancePolicy};
pub use hardware::{HardwareInspector, DirectLinkInterface, UsbControllerInfo, LinkProperties, ConnectionStatus};
