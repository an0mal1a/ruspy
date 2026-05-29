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
pub const WHITE:  &str = "\x1b[97m";