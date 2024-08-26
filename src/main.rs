use slint::slint;

mod lib;
//mod ui;

slint::include_modules!();
fn main() {
    let ui = RxTextEdittor::new().unwrap();
    ui.run().unwrap()
}
