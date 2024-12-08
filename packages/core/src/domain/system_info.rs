use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub processor: ProcessorInfo,
    pub memory: MemoryInfo,
    pub storage: StorageInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorInfo {
    pub name: String,
    pub cores: u32,
    pub frequency_ghz: f64,
    pub architecture: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageInfo {
    pub disks: Vec<DiskInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub disk_type: String,
}
