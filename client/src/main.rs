use std::{io::Read, net::TcpStream};

const SERVER_ADDRESS: &str = "127.0.0.1:1337";

fn handle_server(mut conn: TcpStream) -> Result<(), String> {
    println!("Connected to server: {:#?}", conn);

    let mut finished: bool = false;
    let mut buff = [0; 1024];
    
    while !finished {
        let bytes_read = match conn.read(&mut buff) {
            Ok(b) => b,
            Err(err) => { return Err(err.to_string()) }
        };

        let instruct = String::from_utf8_lossy(&buff[..bytes_read]);

        // This functions return false when we should close the connection
        if !handle_instruct(&instruct) {
            finished = true;
        }
    };

    Ok(())
}

fn handle_instruct(instruct: &str) -> bool {
    if instruct.to_ascii_lowercase() == "q" {
        false // return false to close conn
    } else {
        println!("Command received: {}", instruct);
        true // return true to hold conn
    }
}

fn main() {
    println!("Trying to connect to: {}", SERVER_ADDRESS);

    let conn = TcpStream::connect(SERVER_ADDRESS).expect("Can not connect to the server");
    handle_server(conn);
}
