pub struct Document {
    content: String,
    cursor_position: usize,
    undo_stack: Vec<String>,
    redo_stack: Vec<String>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_position: 0,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn insert_text(&mut self, text: &str, position: usize){
        self.undo_stack.push(self.content.clone());
        self.content.insert_str(position,text);
        self.cursor_position += text.len();
    }

    pub fn remove_text(&mut self, start: usize, end: usize){
        if end >= start{
            self.undo_stack.push(self.content.clone());
            self.content.drain(start..end);
            self.cursor_position = start.min(self.content.len());
        }
    }

    pub fn get_content(&self)->&str{
        &self.content
    }

    pub fn get_cursor_position(&self)->usize{
        self.cursor_position
    }

    pub fn undo(&mut self){
        if let Some(previous_content)=self.undo_stack.pop() {
            self.redo_stack.push(self.content.clone());
            self.content = previous_content;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_text(){
        let mut document = Document::new();
        document.insert_text("Hello ",0);
        document.insert_text("world",5);

        assert_eq!(document.get_content(), "Hello world");
    }
}



