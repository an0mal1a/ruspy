use shared::{
    ClientMessage, Download, FILE_CHUNK_SIZE, FileHeader, utils::{read_message, send_message}
};
use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

pub fn upload(conn: &mut TcpStream) -> Result<bool, String> {
    // Recive file size (or error)
    let msg: ClientMessage = read_message(conn).map_err(|e| e.to_string())?;

    let fileheader: FileHeader = match msg {
        ClientMessage::FileHandler(header) => header,
        ClientMessage::Error(_) => {
            return Ok(true);
        }
        _ => {
            return Ok(true);
        }
    };

    let mut buf = [0u8; FILE_CHUNK_SIZE];
    let mut read_bytes: u64 = 0;

    let mut output = fs::File::create(&fileheader.name).map_err(|e| e.to_string())?;

    while read_bytes < fileheader.size {
        let bytes_read = match conn.read(&mut buf) {
            Ok(0) => return Ok(false), // Server close conn
            Ok(b) => b,
            Err(err) => return Err(err.to_string()),
        };

        output
            .write_all(&buf[..bytes_read])
            .map_err(|e| e.to_string())?;
        read_bytes += bytes_read as u64;
    }

    Ok(true)
}

pub fn download(obj: Download, conn: &mut TcpStream) -> Result<bool, String> {
    let filepath = obj.path; 

    if filepath.is_empty() {
        let _ = send_message(
            conn,
            &ClientMessage::Error("File not specified".to_string()),
        );
        return Ok(true);
    }

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
            let _ = send_message(
                conn,
                &ClientMessage::Error("Errro getting filename".to_string()),
            );
            return Ok(true);
        }
    };

    let msg = ClientMessage::FileHandler(FileHeader {
        name: filename,
        size: file_size,
    });
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
            Err(_) => return Ok(false),
        };

        if bytes_read == 0 {
            break;
        }

        conn.write_all(&buf[..bytes_read])
            .map_err(|e| e.to_string())?;
        read_bytes += bytes_read as u64;
    }

    if obj.delete {
        let _ = fs::remove_file(filepath);
    }


    Ok(true)
}
