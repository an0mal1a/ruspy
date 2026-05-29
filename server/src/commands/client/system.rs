use shared::{ClientMessage, SystemInformation, utils::read_message};


use std::{io::Write, net::TcpStream};

pub fn sysinfo(conn: &mut TcpStream) -> Result<bool, String> {
    conn.write_all(b"sysinfo").expect("Error sending instruct"); // send message

    let msg: ClientMessage = read_message(conn).map_err(|e| e.to_string())?;

    match msg {
        ClientMessage::SystemInformation(info) => {
            pretty_print_sysinfo(&info);
        },
        ClientMessage::Error(err) => {
            println!("client error: {err}");
        }
        _ => {
            println!("unexpected client message");
        }
    }
    
    Ok(true)
}

fn pretty_print_sysinfo(info: &SystemInformation) {
    use crate::constants::{CYAN, DIM, GREEN, RESET, WHITE, YELLOW};

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

    println!(
        "\n\t{DIM}[{RESET}{GREEN}sysinfo{RESET}{DIM}]{RESET} {CYAN}{hostname}{RESET}\n"
    );

    println!("\t{DIM}[{RESET}{WHITE}os{RESET}{DIM}]{RESET}");
    println!("\t\t{YELLOW}name:{RESET}    {}", opt(&info.os.name));
    println!("\t\t{YELLOW}version:{RESET} {}", opt(&info.os.os_version));
    println!("\t\t{YELLOW}kernel:{RESET}  {}", opt(&info.os.kernel_version));

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
        b.memory
            .cmp(&a.memory)
            .then_with(|| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal))
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
