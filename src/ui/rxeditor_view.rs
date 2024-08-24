use slint::prelude::*;
use crate::lib::texteditor::TextEditor;

pub struct TextEditorView {
    app: Application,
    text_editor: TextEditor,
    text_input: TextInput,
}

impl TextEditorView {
    pub fn new(app: Application, text_editor: TextEditor) -> Self {
        let text_input = TextInput::new(&app);

        Self {
            app,
            text_editor,
            text_input,
        }
    }

    pub fn run(&mut self) {
        self.text_input.text.bind(self.text_editor.get_content());

        self.text_input.text_changed.connect(move |text| {
            self.text_editor.insert_text(text);
        });

        self.app.run();
    }
}