// Dependecies
use std::net::{Shutdown, TcpStream};

use shared::{InstructMessage, utils::read_message};

// Internal modules
mod commands;

const SERVER_IP: &str = "127.0.0.1";
const SERVER_ADDRESS: &str = "127.0.0.1:1337";

fn handle_server(mut conn: TcpStream) -> Result<(), String> {
    println!("Connected to server: {:#?}", conn);

    loop {
        let msg: InstructMessage = match read_message(&mut conn) {
            Ok(msg) => msg,
            Err(err) => return Err(err.to_string()),
        };

        // This functions return false when we should close the connection
        match handle_instruct(msg, &mut conn) {
            Ok(b) if b => (),
            Err(err) => {
                println!("An error has ocurred: {}", err);
                break;
            }
            _ => break,
        };
    }

    // This should call again main and restart the execution. NEVER finish
    conn.shutdown(Shutdown::Both).map_err(|e| e.to_string())?;
    Ok(())
}

fn handle_instruct(instruct: InstructMessage, conn: &mut TcpStream) -> Result<bool, String> {
    commands::dispatch(instruct, conn)
}

fn main() {
    println!("Trying to connect to: {}", SERVER_ADDRESS);

    let conn = TcpStream::connect(SERVER_ADDRESS).expect("Can not connect to the server");
    let _ = handle_server(conn);
}
