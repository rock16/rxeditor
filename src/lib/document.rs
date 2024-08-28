use std::collections::LinkedList;

pub struct Document {
    content: String,
    cursor_position: usize,
    undo_stack: LinkedList<String>,
    redo_stack: LinkedList<String>,
    undo_limit: usize,
}

impl Document {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_position: 0,
            undo_stack: LinkedList::new(),
            redo_stack: LinkedList::new(),
            undo_limit: 10
        }
    }

    pub fn insert_text(&mut self, text: &str){
        if self.undo_stack.len()>= self.undo_limit {
            self.undo_stack.pop_front();
        }
        self.undo_stack.push_back(self.content.clone());
        self.content = text.parse().unwrap();
        self.cursor_position += text.len();
        self.redo_stack.clear();
    }

    pub fn remove_text(&mut self, start: usize, end: usize){
        let end = end.min(self.content.len());
        if end >= start{
            if self.undo_stack.len() >= self.undo_limit {
                self.undo_stack.pop_front();
            }
            self.undo_stack.push_back(self.content.clone());
            self.content.drain(start..end);
            self.cursor_position = start.min(self.content.len());
            self.redo_stack.clear();
        }
    }

    pub fn get_content(&self)->&str{
        &self.content
    }

    pub fn get_cursor_position(&self)->usize{
        self.cursor_position
    }

    pub fn undo(&mut self){
        if let Some(previous_content)=self.undo_stack.pop_back() {
            self.redo_stack.push_back(self.content.clone());
            self.content = previous_content;
            self.cursor_position = self.content.len();
        }
    }
    pub fn get_undo_stack_size(&self) -> usize {
        self.undo_stack.len()
    }
    pub fn redo(&mut self){
        if let Some(next_content) = self.redo_stack.pop_back() {
            self.undo_stack.push_back(self.content.clone());
            self.content = next_content;
            self.cursor_position = self.content.len();
        }
    }
    pub fn get_redo_stack_size(&self) -> usize{
        self.redo_stack.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_text(){
        let mut document = Document::new();
        document.insert_text("Hello world");

        assert_eq!(document.get_content(), "Hello world");
    }

    #[test]
    fn test_undo(){
        let mut document = Document::new();
        document.insert_text("Hello ");
        let str = document.get_content().to_string() + "world";
        document.insert_text(&str);
        document.undo();

        assert_eq!(document.get_content(), "Hello ");
    }
    #[test]
    fn test_get_position(){
        let mut document = Document::new();
        document.insert_text("Hello");

        assert_eq!(document.get_cursor_position(), 5)
    }

    #[test]
    fn test_remove_text(){
        let mut document = Document::new();
        document.insert_text("Hello");
        document.remove_text(1, 5);

        assert_eq!(document.get_content(), "H");
    }

    #[test]
    fn test_redo(){
        let mut document = Document::new();
        document.insert_text("Hello ");
        let str = document.get_content().to_string() + "World!";
        document.insert_text(&str);
        document.undo();
        document.redo();
        assert_eq!(document.get_content(), "Hello World!");
    }

}



