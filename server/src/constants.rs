use crate::c2_state::C2State;

pub enum UiEvent {
    AgentConnected(String),
}

pub const RESET: &str = "\x1b[0m";
pub const DIM: &str = "\x1b[2m";
pub const BOLD: &str = "\x1b[1m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const RED: &str = "\x1b[31m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[97m";
pub const OUTPATH: &str = "./DATA";

pub fn build_prompt(state: &C2State) -> (String, String) {
    let count = state.agent_count();
    let module = state.current_mod();

    let agents_color = match count {
        0 => RED,
        1..=4 => YELLOW,
        _ => CYAN,
    };

    let raw = format!("C&C [agents:{count}] [mod:{module}] ❯ ");

    let styled = format!(
        "{BOLD}{RED}C&C{RESET} \
         {DIM}[{RESET}{agents_color}agents:{count}{RESET}{DIM}]{RESET} \
         {DIM}[{RESET}{WHITE}mod:{module}{RESET}{DIM}]{RESET} \
         {BOLD}{CYAN}❯{RESET} "
    );

    (raw, styled)
}
