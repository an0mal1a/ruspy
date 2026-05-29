use shared::{SystemInformation, OsInformation, HardwareInformation, MemoryInformation, ProcessInformation, ClientMessage, utils::send_message};
use std::net::TcpStream;

use sysinfo::{
    Disks, Networks, System, 
};

// ---- sysInfo ---------------------------

fn get_system_information() -> SystemInformation {
    let mut sys = System::new();
    sys.refresh_all();

    let os_info = OsInformation {
        name: System::name(),
        hostname: System::host_name(),
        kernel_version: System::kernel_version(),
        os_version: System::os_version()
    };

    let hwd_info = HardwareInformation {
        cpu_brand: sys.cpus().first().map(|cpu| cpu.brand().to_string()).unwrap_or_else(|| "unknown".to_string()),
        cpu_count: sys.cpus().len()
    };

    let mem_info = MemoryInformation {
        total_ram: sys.total_memory(),
        used_ram: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
    };

    let mut processes: Vec<ProcessInformation> = sys
        .processes()
        .iter()
        .map(|(pid, process)| ProcessInformation {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().to_string(),
            cpu_usage: process.cpu_usage(),
            memory: process.memory(),
        })
        .collect();

    // processes.sort_by(|a, b| b.cpu_usage.total_cmp(&a.cpu_usage));


    SystemInformation { os: os_info, hardware: hwd_info, memory: mem_info, processes }
}   

pub fn sysinfo(conn: &mut TcpStream) -> Result<bool, String> {
    let info: SystemInformation = get_system_information();

    let msg = ClientMessage::SystemInformation(info);
    match send_message(conn, &msg) {
        Ok(_) => Ok(true),
        Err(e) => return Err(e.to_string()) // ive should handle this better, the server can get stuck waiting for a msg
    }
}
// ---- sysInfo ---------------------------