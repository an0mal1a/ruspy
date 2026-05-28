use std::{io::Write, net::TcpStream};

pub fn run(conn: &mut TcpStream) -> Result<bool, String>{
    conn.write_all(b"pong").expect("I can not send the response to server...");
    Ok(true)
}