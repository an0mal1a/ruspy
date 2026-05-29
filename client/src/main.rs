// Dependecies
use std::{io::Read, net::{TcpStream, Shutdown}};

// Internal modules
mod commands;

const SERVER_ADDRESS: &str = "127.0.0.1:1337";

fn handle_server(mut conn: TcpStream) -> Result<(), String> {
    println!("Connected to server: {:#?}", conn);

    let mut buff = [0; 1024];
    
    loop {
        let bytes_read = match conn.read(&mut buff) {
            Ok(0) => { 
                println!("Server close connection...");
                break;
            }

            Ok(b) => b,
            Err(err) => { return Err(err.to_string()) }
        };

        let instruct = String::from_utf8_lossy(&buff[..bytes_read]);

        // This functions return false when we should close the connection
        match handle_instruct(instruct.trim().split_whitespace().collect(), &mut conn) {
            Ok(b) if b => (),
            Err(err) => { println!("An error has ocurred: {}", err); break; }
            _ => break
        };
    }

    // This should call again main and restart the execution. NEVER finish
    conn.shutdown(Shutdown::Both).map_err(|e| e.to_string())?;
    Ok(())
}

fn handle_instruct(instruct: Vec<&str>, conn: &mut TcpStream) -> Result<bool, String> {
    commands::dispatch(instruct, conn)
}

fn main() {
    println!("Trying to connect to: {}", SERVER_ADDRESS);

    let conn = TcpStream::connect(SERVER_ADDRESS).expect("Can not connect to the server");
    let _ = handle_server(conn);
}
