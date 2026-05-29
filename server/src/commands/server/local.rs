use crate::constants::{RESET, RED, YELLOW, DIM, WHITE, CYAN};

use std::io::Write;
use std::{env, path::Path};
use std::{fs, io};



pub fn local_cd(instruct: &[&str]) -> Result<bool, String> {
    // Remove lcd from instruct
    let path = match instruct.get(1) {
        Some(p) => p,
        None => {
            println!("\n\t{DIM}[{RESET}{RED}err:lcd{RESET}{DIM}]{RESET} {YELLOW}Missing path:{RESET} {WHITE}{}{RESET}\n", instruct.join(" "));
            return Ok(false)
        }
    };

    // Check if path is correct
    if !Path::new(path).exists(){
        println!("\n\t{DIM}[{RESET}{RED}err:lcd{RESET}{DIM}]{RESET} {YELLOW}Path dont exist:{RESET} {WHITE}{}{RESET}\n", path);
        return Ok(false)
    }

    // Change directory
    Ok(env::set_current_dir(path).is_ok())
}


pub fn local_list() -> Result<bool, String> {
    let current_dir = env::current_dir().map_err(|e| e.to_string())?;
    let entries = fs::read_dir(current_dir.clone()).map_err(|e| e.to_string())?;

    println!("\n\t{DIM}[{RESET}{WHITE}ls:{RESET}{DIM}]{RESET} {CYAN}{}{RESET}\n", current_dir.display());

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let metadata = entry.metadata().map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();

        if metadata.is_dir() {
            println!("\t\t{}[DIR]{} {}{}{}",YELLOW,RESET,CYAN,name,RESET);
        } else {
            println!("\t\t      {}{}{}",WHITE,name,RESET);
        }
    }

    println!();
    Ok(true)
}
 
pub fn local_pwd() -> Result<bool, String> {
    let current_path = env::current_dir().map_err(|e| e.to_string())?;
    println!("\n\t{DIM}[{RESET}{WHITE}cwd:{RESET}{DIM}]{RESET} {CYAN}{}{RESET}\n",current_path.display());
    Ok(true)
}


pub fn clear_console() -> Result<bool, String> { 
    print!("\x1B[2J\x1B[1;1H"); 
    io::stdout().flush().expect("Cannot flush output");
    Ok(true) 
}