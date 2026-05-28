use std::net::TcpStream;

pub mod ping;

pub fn dispatch(instruct: &str, conn: &mut TcpStream) -> Result<bool, String> {    
    match instruct {
        "q" => Ok(false),
        "ping" => ping::run(conn),

        // No registered command
        _ => Ok(true)
    }
}