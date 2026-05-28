use std::io::{Read, Write};
use std::net::TcpStream;

pub fn run(conn: &mut TcpStream) -> Result<bool, String> {
    let mut buff = [0; 1024];
    
    // Send instruct
    conn.write(b"ping").expect("Error sending instruct");
    
    // Recive
    let bytes_read = match conn.read(&mut buff) {
        Ok(0) => {
            println!("Client closed connection...");
            return Ok(false);
        }
        Ok(b) => b,
        Err(e) => return Err(e.to_string()),
    };

    println!(
        "Response from client: {}",
        String::from_utf8_lossy(&buff[..bytes_read])
    );

    Ok(true)
}