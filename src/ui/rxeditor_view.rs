use slint::{ComponentHandle, PlatformError};
use crate::lib::texteditor::TextEditor;


slint::include_modules!();

pub struct TextEditorView {
    app: RxTextEdittor,
    text_editor: TextEditor,
    //text_input: TextInput,
}

impl TextEditorView
{
    pub fn new(text_editor: TextEditor) -> Self {
        let app = RxTextEdittor::new().unwrap();

        Self {
            app,
            text_editor,
            //text_input,
        }
    }

    pub fn run(&mut self) {
        //self.text_input.text.bind(self.text_editor.get_content());

        /*
        self.text_input.text_changed.connect(move |text| {
            self.text_editor.insert_text(text);
        });

         */
        let ui_handle = self.app.as_weak();
        self.app.global::<TextContent>().(move |new_txt|{
            self.text_editor.insert_text(new_txt);
            let word_count = new_txt.split_whitespace().count();
            let ui = ui_handle.unwrap();
            ui.set_word_count(format!("{}", word_count));
        });
        self.app.run().expect("Unable to run textEdittor");
    }
}