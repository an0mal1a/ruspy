use std::{io::{Read, Write}, net::{TcpStream, Shutdown}};

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
        if !handle_instruct(&instruct, &mut conn) {
            break
        }
    }

    conn.shutdown(Shutdown::Both).map_err(|e| e.to_string())?;
    Ok(())
}

fn handle_instruct(instruct: &str, conn: &mut TcpStream) -> bool {
    match instruct {
        "q" => false,
        "hello" => {
            conn.write_all(b"World").expect("I can not send the response to server...");
            return true;
        },
        _ => {
            println!("The command is not recognized: {}", instruct);
            return true;
        }
    }
}

fn main() {
    println!("Trying to connect to: {}", SERVER_ADDRESS);

    let conn = TcpStream::connect(SERVER_ADDRESS).expect("Can not connect to the server");
    handle_server(conn);
}
