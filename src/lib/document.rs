pub struct Document {
    content: String,
    cursor_position: usize,
}

impl Document {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_position: 0,
        }
    }

    pub fn insert_text(&mut self, text: &str, position: usize){
        self.content.insert_str(position,text);
        self.cursor_position += text.len();
    }

    pub fn remove_text(&mut self, start: usize, end: usize){
        if end >= start{
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

}

