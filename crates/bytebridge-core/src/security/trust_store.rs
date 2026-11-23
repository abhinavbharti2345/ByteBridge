use crate::device::PairedDevice;
use std::collections::HashMap;
use std::sync::RwLock;

pub trait TrustStore: Send + Sync {
    fn is_paired(&self, public_key: &[u8; 32]) -> bool;
    fn add_paired_device(&self, device: PairedDevice) -> Result<(), String>;
    fn revoke_device(&self, public_key: &[u8; 32]) -> Result<(), String>;
    fn list_paired_devices(&self) -> Vec<PairedDevice>;
}

/// In-memory trust store for tests and staging
pub struct MemoryTrustStore {
    devices: RwLock<HashMap<[u8; 32], PairedDevice>>,
}

impl MemoryTrustStore {
    pub fn new() -> Self {
        Self {
            devices: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryTrustStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TrustStore for MemoryTrustStore {
    fn is_paired(&self, public_key: &[u8; 32]) -> bool {
        self.devices.read().unwrap().contains_key(public_key)
    }

    fn add_paired_device(&self, device: PairedDevice) -> Result<(), String> {
        self.devices.write().unwrap().insert(device.public_key, device);
        Ok(())
    }

    fn revoke_device(&self, public_key: &[u8; 32]) -> Result<(), String> {
        self.devices.write().unwrap().remove(public_key);
        Ok(())
    }

    fn list_paired_devices(&self) -> Vec<PairedDevice> {
        self.devices.read().unwrap().values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_store_lifecycle() {
        let store = MemoryTrustStore::new();
        let key = [7u8; 32];
        assert!(!store.is_paired(&key));

        let device = PairedDevice {
            device_name: "Laptop-B".to_string(),
            os: "Linux".to_string(),
            public_key: key,
            paired_timestamp: 1732360000,
        };
        store.add_paired_device(device).unwrap();
        assert!(store.is_paired(&key));

        store.revoke_device(&key).unwrap();
        assert!(!store.is_paired(&key));
    }
}
