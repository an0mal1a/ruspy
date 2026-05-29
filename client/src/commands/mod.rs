use std::net::TcpStream;

pub mod system;
pub mod ping;

pub fn dispatch(instruct: &str, conn: &mut TcpStream) -> Result<bool, String> {    
    match instruct {
        "q" => Ok(false),
        "ping" => ping::run(conn),

        // System interaction
        "sysinfo" => system::sysinfo(conn),
        "check" => system::check_privileges(conn),

        // No registered command
        _ => Ok(true)
    }
}