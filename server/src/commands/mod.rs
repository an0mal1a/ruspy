use crate::constants::{YELLOW, RED, BOLD, WHITE, DIM, RESET};
use std::{net::TcpStream};

pub mod client;
pub mod server;

pub fn dispatch_client(instruct: &str, conn: &mut TcpStream) -> Result<bool, String> {    
    match instruct {
        "ping" => client::ping::run(conn),
        _ => {
            println!("\n\t{}[{}!{}>{}]{} Incorrect instruction: {}{}\n", YELLOW, RED, BOLD, YELLOW, YELLOW, RESET, instruct);
            Ok(true)
        }
    }
}

pub fn dispatch_server(instruct: Vec<&str>) -> Result<bool, String> {    
    match instruct.first() {
        // Local commands
        Some(&"lcd") => server::local::local_cd(&instruct),
        Some(&"lls") => server::local::local_list(),
        Some(&"lpwd") => server::local::local_pwd(),
        Some(&"clear") => server::local::clear_console(),

        // Session commands

        // Control commands
        _ => {
            println!("\n\t{DIM}[{RESET}{RED}err{RESET}{DIM}]{RESET} {YELLOW}Incorrect instruction:{RESET} {WHITE}{}{RESET}\n", instruct.join(" "));
            Ok(true)
        }
    }
}