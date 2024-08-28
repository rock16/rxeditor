use slint::slint;
use crate::lib::texteditor::TextEditor;
use crate::ui::rxeditor_view::TextEditorView;


mod lib;
mod ui;


fn main() {

    let text_edittor = TextEditor::new();
    let mut tx_view = TextEditorView::new(text_edittor);

    tx_view.callback();

    tx_view.run().unwrap();

    /*
    let text_edittor = TextEditor::new();
    let mut tx_view = TextEditorView::new(text_edittor);

    let window = RxTextEdittor::new().unwrap();

    let ui_handle = window.as_weak();

    window.global::<TextContent>().on_text_editted(move |new_text| {
        let ui = ui_handle.unwrap();
        let word_count = new_text.split_whitespace().count();
        ui.set_word_count(word_count as i32);
        println!("{}", new_text);
    });

    window.on_button_clicked(move ||{
        println!("New button clicked");
    });



    window.run().unwrap();

     */
}
