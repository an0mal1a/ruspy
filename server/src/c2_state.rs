use std::{fs, net::TcpStream, sync::{Arc, Mutex}};
use crate::constants::OUTPATH;

pub struct C2State {
    pub agents:         Arc<Mutex<Vec<AgentConnection>>>,
    pub active_mod:     Arc<Mutex<String>>,
    pub active_session: Arc<Mutex<Option<usize>>>
}

#[derive(Debug)]
pub struct AgentConnection {
    pub id: usize,
    pub ip: String,
    pub path: String,
    pub conn: TcpStream
}

impl C2State {
    pub fn new() -> Self {
        Self {
            agents:         Arc::new(Mutex::new(Vec::new())),
            active_mod:     Arc::new(Mutex::new("manager".to_string())),
            active_session: Arc::new(Mutex::new(None)),
        }
    }

    pub fn agent_count(&self) -> usize {
        self.agents.lock().unwrap().len()
    }

    pub fn id_exists(&self, id: usize) -> bool {
        self.agents.lock().unwrap().iter().any(|a| a.id == id)
    }

    pub fn current_mod(&self) -> String {
        self.active_mod.lock().unwrap().clone()
    }

    pub fn set_mod(&self, module: &str) {
        *self.active_mod.lock().unwrap() = module.to_string()
    }

    pub fn add_agent(&self, ip: &str, conn: TcpStream) {
        let next_id = self.agent_count() + 1;
        let path = format!("{}/{}", OUTPATH, Self::sanitize_path_component(ip));
        let _ = fs::create_dir(&path);
        self.agents.lock().unwrap().push(AgentConnection { id: next_id, ip: ip.to_string(), conn, path: path});
    }

    pub fn remove_agent(&self, id: usize) {
        self.agents.lock().unwrap().retain(|a| a.id != id);
    }

    pub fn active_session(&self) -> Option<usize> {
        *self.active_session.lock().unwrap()
    }

    pub fn set_active_session(&self, id: usize) -> Result<(), String> {
        if !self.id_exists(id) {
            return Err(format!("Session {} not found", id));
        }

        let mut active_session = self.active_session.lock().map_err(|e| e.to_string())?;
        *active_session = Some(id);

        Ok(())
    }

    pub fn get_active_agent(&self) -> Result<AgentConnection, String> {
        let active_session = self.active_session.lock().map_err(|e| e.to_string())?;

        let active_id = match *active_session {
            Some(id) => id,
            None => return Err("No active session".to_string()),
        };

        drop(active_session);

        let agents = self.agents.lock().map_err(|e| e.to_string())?;
        let agent = agents
            .iter()
            .find(|a| a.id == active_id)
            .ok_or_else(|| format!("Session {} not found", active_id))?;

        Ok(AgentConnection {
            id: agent.id,
            ip: agent.ip.clone(),
            path: agent.path.clone(),
            conn: agent.conn.try_clone().map_err(|e| e.to_string())?,
        })
    }

    pub fn get_active_path(&self) -> String {
        match self.get_active_agent() {
            Ok(a) => a.path,
            Err(e) => e.to_string()
        }
    }

    fn sanitize_path_component(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
                _ => c,
            })
            .collect()
    }

}