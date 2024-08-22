use crate::lib::document::Document;

pub struct TextEditor{
    document: Document,
}

impl TextEditor {
    pub fn new()->Self{
        TextEditor{
            document: Document::new(),
        }
    }

    pub fn insert_text(&mut self, text: &str) {
        self.document.insert_text(text, self.document.get_cursor_position());
    }

    pub fn get_content(&self) -> &str {
        self.document.get_content()
    }

    pub fn remove_text(&mut self, length: usize) {
        let start = self.document.get_cursor_position();
        let end= start - length;
        let end = end.max(0);
        self.document.remove_text(end, start);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_text(){
        let mut texteditor = TextEditor::new();
        texteditor.insert_text("Hello");
        texteditor.insert_text(" ");
        texteditor.insert_text("World!");

        texteditor.remove_text(2);

        assert_eq!(texteditor.get_content(), "Hello Worl");
    }
}