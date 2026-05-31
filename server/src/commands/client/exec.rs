use crate::{constants::{self, DIM, CYAN, GREEN, RED, RESET, WHITE, YELLOW}};
use shared::{ClientMessage, InstructMessage, utils::{read_message, send_message}};
use std::net::TcpStream;


pub fn handle_command(instruct: String, conn: &mut TcpStream) -> Result<bool, String> {
    match send_message(conn, &InstructMessage::Exec(instruct)) 
    
    {
        Ok(_) => (),
        Err(e) => {
            println!("\n\t{DIM}[{RESET}{RED}err:exec{RESET}{DIM}]{RESET} {YELLOW}instruct hasnt been sended:{RESET} {WHITE}{}{RESET}\n", e);
            return Err(e.to_string())
        }
    };

    let msg: ClientMessage = read_message(conn).map_err(|e| e.to_string())?;
    
    let out = match msg {
        ClientMessage::ShellOutput(out) => out,
        ClientMessage::Error(e) => {
            println!("\n\t{DIM}[{RESET}{RED}err:exec{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n", e);
            return Err(e)
        },
        _ => {
            println!("\n\t{DIM}[{RESET}{RED}err:exec{RESET}{DIM}]{RESET} {YELLOW}Response not recognized{RESET}\n");
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


pub fn run(conn: &mut TcpStream, instruct: &[&str]) -> Result<bool, String> {
    let command: String = match instruct.get(1..) {
        Some(c) => c.join(" "),
        None => {
            println!(
                "\n\t{DIM}[{RESET}{RED}err:exec{RESET}{DIM}]{RESET} {YELLOW}Command not specified{RESET}\n"
            );
            return Ok(true);

        }
    };

    match handle_command(command, conn) 
    {
        Ok(b) if b => (),
        Err(err) => 
        {
            println!("An error has ocurred: {}", err);
            return Ok(true)
        }
        _ => { return Ok(true)}
    };


    Ok(true)
}