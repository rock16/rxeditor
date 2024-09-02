use std::sync::{Arc, Mutex};
use crate::lib::texthistory::TextHistory;

pub struct AppState {
    pub text_history: Arc<Mutex<TextHistory>>,
    pub is_programmatic_change: Arc<Mutex<bool>>,
}

impl AppState {
    pub fn new()->Self{
        let text_history = Arc::new(Mutex::new(TextHistory::new()));
        let is_programmatic_change = Arc::new(Mutex::new(false));
        AppState{
            text_history,
            is_programmatic_change,
        }
    }

    pub fn get_text_history(&self)->Arc<Mutex<TextHistory>>{
        self.text_history.clone()
    }
    pub fn get_is_programmatic_change(&self)->Arc<Mutex<bool>>{
        self.is_programmatic_change.clone()
    }
}