use crate::constants::{YELLOW, RED, BOLD, RESET};
use std::net::TcpStream;

pub mod ping;

pub fn dispatch(instruct: &str, conn: &mut TcpStream) -> Result<bool, String> {    
    match instruct {
        "ping" => ping::run(conn),
        _ => {
            println!("\n\t{}[{}!{}>{}]{} Incorrect instruction: {}{}\n", YELLOW, RED, BOLD, YELLOW, YELLOW, RESET, instruct);
            Ok(true)
        }
    }
}