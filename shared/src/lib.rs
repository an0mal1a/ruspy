use serde::{Serialize, Deserialize};

pub mod utils;

// Client/Server messages
#[derive(Serialize, Deserialize)]
pub enum ClientMessage  {
    SystemInformation(SystemInformation),
    Pong, 
    Error(String)
}

// AdminStruct
#[derive(Serialize, Deserialize)]
pub enum Privilege {
    Admin,
    User
}

//  System information
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInformation {
    pub os: OsInformation,
    pub hardware: HardwareInformation,
    pub memory: MemoryInformation,
    pub processes: Vec<ProcessInformation>,
    pub disks: Vec<DiskInformation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsInformation {
    pub name: Option<String>,
    pub hostname: Option<String>,
    pub kernel_version: Option<String>,
    pub os_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareInformation {
    pub cpu_brand: String,
    pub cpu_count: usize
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  MemoryInformation {
    pub total_ram: u64,
    pub used_ram: u64,
    pub total_swap: u64,
    pub used_swap: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInformation {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInformation {
    pub name: String,
    pub file_system: String,
    pub total_space: u64,
    pub available_space: u64,
    pub is_removable: bool,
    pub is_read_only: bool,
    pub device_path: String,
}