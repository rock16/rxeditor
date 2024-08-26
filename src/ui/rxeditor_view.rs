use slint::{ComponentHandle, PlatformError};
use crate::lib::texteditor::TextEditor;
//use slint::

pub struct TextEditorView<T> {
    app: T,
    text_editor: TextEditor,
    //text_input: TextInput,
}

impl<T> TextEditorView<T>
where
    T: ComponentHandle,
{
    pub fn new(app: T, text_editor: TextEditor) -> Self {
        //let text_input = TextInput::new(&app);

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
        self.app.run().expect("Unable to run textEdittor");
    }
}