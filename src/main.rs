use slint::slint;
use crate::lib::texteditor::TextEditor;
use crate::ui::rxeditor_view::TextEditorView;

mod lib;
mod ui;

slint::include_modules!();
fn main() {
    let ui = RxTextEdittor::new().unwrap();
    let text_edittor = TextEditor::new();
    let mut tx_view = TextEditorView::new(ui, text_edittor);
    tx_view.run();
}
