use std::fs;
use std::sync::{Arc, mpsc, Mutex};
use futures::channel::mpsc::Receiver;
use futures::StreamExt;
use rfd::FileDialog;
use slint::SharedString;
use tokio::time::{sleep, Duration, Instant};
use crate::lib::texthistory::TextHistory;
use crate::RxTextEdittor;
pub async fn debouncer(
    mut receiver: Receiver<String>,
    mut debounce_time: Duration,
    is_programmatic_change: Arc<Mutex<bool>>,
    text_history: Arc<Mutex<TextHistory>>,
    ui_handle: slint::Weak<RxTextEdittor>,
) {
    let mut last_text = String::new();
    let start = Instant::now();
    while let Some(text) = receiver.next().await {
        //sleep(debounce_time).await;

        if *is_programmatic_change.lock().unwrap() {
            continue; // skip processing
        }
        let elapsed_time = Instant::now().duration_since(start);

        if elapsed_time - debounce_time > Duration::from_millis(500) && last_text != text {
            debounce_time = elapsed_time;
            last_text = text.clone();


            let mut history = text_history.lock().unwrap();
            history.add_change(text);

            if let Some(ui) = ui_handle.upgrade() {
                ui.set_undo_enabled(history.can_undo());
                ui.set_redo_enabled(history.can_redo());
                ui.set_content(SharedString::from(history.history[history.current_index].clone()));
            }
        }
    }
}

pub fn save_as(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = FileDialog::new()
        .set_directory("/")
        .add_filter("Text files", &["txt"])
        .save_file() {
        fs::write(path, content)?;
        println!("File saved successfully");
    } else {
        println!("Save operation canceled");
    }
    Ok(())
}