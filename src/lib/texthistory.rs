pub struct TextHistory {
    pub history: Vec<String>,
    pub current_index: usize,
}

impl TextHistory {
    pub fn new() -> Self{
        TextHistory{
            history: vec![String::new()],
            current_index: 0,
        }
    }

    pub fn undo(&mut self) -> Option<String> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(self.history[self.current_index].clone())
        } else {
            None
        }
    }
    pub fn add_change(&mut self, text: String) {
        if self.current_index == 0 || text != self.history[self.current_index] {
            self.history.truncate(self.current_index + 1);
            println!("{}", text);
            self.history.push(text);
            self.current_index += 1;
        }
    }

    pub fn redo(&mut self) -> Option<String> {
        if self.current_index < self.history.len() - 1 {
            self.current_index += 1;
            Some(self.history[self.current_index].clone())
        } else {
            None
        }
    }

    pub fn can_undo(&self) -> bool {
        self.current_index > 0
    }

    pub fn can_redo(&self) -> bool {
        self.current_index < self.history.len() - 1
    }
}