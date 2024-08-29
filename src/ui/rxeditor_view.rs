use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use slint::{ComponentHandle, PlatformError};
use crate::lib::texteditor::TextEditor;
slint::include_modules!();


pub struct TextEditorView {
    ui: RxTextEdittor,
    text_editor: Rc<RefCell<TextEditor>>,
}

impl TextEditorView
{
    pub fn new(text_editor: TextEditor) -> Self {
        let ui = RxTextEdittor::new().unwrap();
        let text_editor= Rc::new(RefCell::new(text_editor));
        Self {
            ui,
            text_editor,
        }
    }

    pub(crate) fn callback(&mut self){
        let ui_handler = self.ui.as_weak();

        // Text input callback
        let tx = Rc::clone(&self.text_editor);
        self.ui.global::<TextContent>().on_text_editted(move |new_txt| {
            let mut tx_editor = tx.borrow_mut();
            tx_editor.insert_text(&new_txt);
            let ui = ui_handler.upgrade().unwrap();
            Self::set_redo_and_undo(ui, tx_editor);
        });

        // Undo callbacks: react to undo button pressed
        let ui_handler = self.ui.as_weak();
        let tx_editor_undo_handler = Rc::clone(&self.text_editor);
        self.ui.global::<MyMenuCallback>().on_undo(move ||{
            let mut tx_editor = tx_editor_undo_handler.borrow_mut();
            tx_editor.undo();
            let ui = ui_handler.upgrade().unwrap();
            ui.set_content(tx_editor.get_content().into());
            Self::set_redo_and_undo(ui, tx_editor);

        });

        // Redo callbacks: react to Redo button
        let ui_redo_handler = self.ui.as_weak();
        let tx_editor_redo_handler = Rc::clone(&self.text_editor);
        self.ui.global::<MyMenuCallback>().on_redo(move ||{
            let mut tx_editor = tx_editor_redo_handler.borrow_mut();
            tx_editor.redo();
            let ui = ui_redo_handler.upgrade().unwrap();
            ui.set_content(tx_editor.get_content().into());
            Self::set_redo_and_undo(ui, tx_editor);
        });
    }

    fn set_redo_and_undo(ui: RxTextEdittor, tx_editor: RefMut<TextEditor>) {
        // enable undo button if there are items on the undo stack
        ui.set_undo_enabled(tx_editor.should_undo());
        // enable redo button if there are items on the redo stack
        ui.set_redo_enabled(tx_editor.should_redo());
    }

    pub fn run(mut self)-> Result<(), PlatformError>{
        self.ui.run()
    }
}