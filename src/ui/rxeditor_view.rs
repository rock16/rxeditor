use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::lib::texthistory::TextHistory;

pub struct Tab{
    pub text_history: Arc<Mutex<TextHistory>>,
    pub is_programmatic_change: Arc<Mutex<bool>>,
    current_file_path: Arc<Mutex<Option<PathBuf>>>
}

impl Tab{
    fn new() -> Self{
        let text_history = Arc::new(Mutex::new(TextHistory::new()));
        let is_programmatic_change = Arc::new(Mutex::new(false));
        let current_file_path = Arc::new(Mutex::new(None));
        Tab{
            text_history,
            is_programmatic_change,
            current_file_path,
        }
    }
}

pub struct AppState {
    pub tabs: Vec<Tab>,
    current_index: usize
}

impl AppState {
    pub fn new()->Self{
        let mut tab = Tab::new();
        let mut tabs: Vec<Tab> = Vec::new();
        tabs.push(tab);
        let current_index = 0;
        AppState{
            tabs,
            current_index
        }
    }

    pub fn get_text_history(&self)->Arc<Mutex<TextHistory>>{
        self.tabs[self.current_index].text_history.clone()
    }
    pub fn get_is_programmatic_change(&self)->Arc<Mutex<bool>>{
        self.tabs[self.current_index].is_programmatic_change.clone()
    }
    pub fn get_current_file_path(&self) -> Arc<Mutex<Option<PathBuf>>>{
        self.tabs[self.current_index].current_file_path.clone()
    }
    pub fn set_current_file_path(&mut self, path: PathBuf) {
        let current_path = Arc::new(Mutex::new(Some(path)));
        self.tabs[self.current_index].current_file_path = current_path;
    }
}