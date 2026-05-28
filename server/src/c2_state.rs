use std::sync::{Arc, Mutex};

pub struct C2State {
    pub agents:      Arc<Mutex<Vec<String>>>,
    pub active_mod:  Arc<Mutex<String>>,
}

impl C2State {
    pub fn new() -> Self {
        Self {
            agents:     Arc::new(Mutex::new(Vec::new())),
            active_mod: Arc::new(Mutex::new("".to_string())),
        }
    }

    pub fn agent_count(&self) -> usize {
        self.agents.lock().unwrap().len()
    }

    pub fn current_mod(&self) -> String {
        self.active_mod.lock().unwrap().clone()
    }

    pub fn set_mod(&self, module: &str) {
        *self.active_mod.lock().unwrap() = module.to_string()
    }

    pub fn add_agent(&self, id: &str) {
        self.agents.lock().unwrap().push(id.to_string());
    }

    pub fn remove_agent(&self, id: &str) {
        self.agents.lock().unwrap().retain(|a| a != id);
    }

}