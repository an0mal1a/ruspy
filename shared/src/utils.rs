use std::io::{Read, Write};
use std::net::TcpStream;

pub fn send_message<T: serde::Serialize>(stream: &mut TcpStream, message: &T) -> Result<(), Box<dyn std::error::Error>> {
    let payload = serde_json::to_vec(message)?;

    let len = payload.len() as u32;
    let len_bytes = len.to_be_bytes();

    stream.write_all(&len_bytes)?;
    stream.write_all(&payload)?;

    Ok(())
}

pub fn read_message<T: serde::de::DeserializeOwned>(stream: &mut TcpStream) -> Result<T, Box<dyn std::error::Error>> {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes)?;

    let len = u32::from_be_bytes(len_bytes) as usize;

    let mut payload = vec![0u8; len];
    stream.read_exact(&mut payload)?;

    let message = serde_json::from_slice(&payload)?;

    Ok(message)
}