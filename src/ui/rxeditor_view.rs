use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::lib::texthistory::TextHistory;

pub struct AppState {
    pub text_history: Arc<Mutex<TextHistory>>,
    pub is_programmatic_change: Arc<Mutex<bool>>,
    current_file_path: Arc<Mutex<Option<PathBuf>>>,
}

impl AppState {
    pub fn new()->Self{
        let text_history = Arc::new(Mutex::new(TextHistory::new()));
        let is_programmatic_change = Arc::new(Mutex::new(false));
        let current_file_path = Arc::new(Mutex::new(None));
        AppState{
            text_history,
            is_programmatic_change,
            current_file_path,
        }
    }

    pub fn get_text_history(&self)->Arc<Mutex<TextHistory>>{
        self.text_history.clone()
    }
    pub fn get_is_programmatic_change(&self)->Arc<Mutex<bool>>{
        self.is_programmatic_change.clone()
    }
    pub fn get_current_file_path(&self) -> Arc<Mutex<Option<PathBuf>>>{
        self.current_file_path.clone()
    }
    pub fn set_current_file_path(&mut self, path: PathBuf) {
        let current_path = Arc::new(Mutex::new(Some(path)));
        self.current_file_path = current_path;
    }
}