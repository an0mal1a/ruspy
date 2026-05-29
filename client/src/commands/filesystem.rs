use std::{fs, io::{Read, Write}, net::TcpStream, path::Path};
use shared::{ClientMessage, FILE_CHUNK_SIZE, FileHeader, utils::send_message};

pub fn download(instruct: &[&str], conn: &mut TcpStream) -> Result<bool, String> {
    let filepath = match instruct.get(1..instruct.len()) {
        Some(raw) => raw.join(" "),
        None => {
            let _ = send_message(conn, &ClientMessage::Error("File not specified".to_string()));
            return Ok(true);
        }
    };

    // Check if file exists and is a file
    let filepath = Path::new(&filepath);
    let metadata = match filepath.metadata() {
        Ok(m) => m,
        Err(_) => {
            let _ = send_message(conn, &ClientMessage::Error("File not found".to_string()));
            return Ok(true);
        }
    };

    if !metadata.is_file() {
        let _ = send_message(conn, &ClientMessage::Error("Path is not file".to_string()));
        return Ok(true);
    }

    let file_size = metadata.len();
    let filename = match filepath.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => {
            let _ = send_message(conn, &ClientMessage::Error("Errro getting filename".to_string()));
            return Ok(true);
        }
    };

    let msg = ClientMessage::FileDownload(FileHeader { name: filename, size: file_size });
    let _ = send_message(conn, &msg);

    // Read in chunks and send it
    let mut file_handle = match fs::File::open(filepath) {
        Ok(h) => h,
        Err(_) => {
            let _ = send_message(conn, &ClientMessage::Error("Can not open file".to_string()));
            return Ok(true);
        }
    };

    let mut read_bytes: u64 = 0;
    let mut buf = [0u8; FILE_CHUNK_SIZE];
    
    while read_bytes < file_size {
        let bytes_read: usize = match file_handle.read(&mut buf) {
            Ok(b) => b,
            Err(_) => { return Ok(false) }
        };

        if bytes_read == 0 {
            break;
        }

        conn.write_all(&buf[..bytes_read]).map_err(|e| e.to_string())?;
        read_bytes += bytes_read as u64;
    };


    Ok(true)
}