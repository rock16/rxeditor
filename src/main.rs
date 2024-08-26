use slint::slint;
use crate::lib::texteditor::TextEditor;
use crate::ui::rxeditor_view::TextEditorView;

mod lib;
mod ui;

fn main() {
    let text_edittor = TextEditor::new();
    let mut tx_view = TextEditorView::new(text_edittor);
    tx_view.run();
}
