use std::fs;
use std::path::PathBuf;
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

        if elapsed_time - debounce_time > Duration::from_millis(900) && last_text != text {
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

pub fn save_as(content: &str, current_file_path: &Arc<Mutex<Option<PathBuf>>>) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = FileDialog::new()
        .set_directory("/")
        .add_filter("Text files", &["txt"])
        .save_file() {
        if let Err(e) = fs::write(&path, &content){
            eprintln!("Error saving file: {}", e);
        } else {
            *current_file_path.lock().unwrap() = Some(path);
            println!("File saved successfully");
        }
    } else {
        println!("Save operation canceled");
    }
    Ok(())
}

pub fn open_file(current_file_path: &Arc<Mutex<Option<PathBuf>>>) -> Result<String, Box<dyn std::error::Error>> {
    let current_path = current_file_path.lock().unwrap();
    if let Some(path) = FileDialog::new()
        .add_filter("text", &["txt"])
        .set_directory("/")
        .pick_file(){
        let path_clone = path.clone();
        let content = fs::read(path)?;
        *current_path = Some(path_clone);
        Ok(String::from_utf8(content)?)
    } else {
        println!("open operation cancelled");
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Operation cancelled")))
    }
}

pub fn save_file(content: &str, current_file_path: &Arc<Mutex<Option<PathBuf>>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut path = current_file_path.lock().unwrap();

    if let Some(file_path) = path.as_ref() {
        // We have a current file path, so save directly
        fs::write(file_path, content)?;
        println!("File saved successfully");
    } else {
        // No current file path, so open a save dialog
        if let Some(new_path) = FileDialog::new()
            .set_directory("/")
            .add_filter("Text Files", &["txt"])
            .save_file() {
            fs::write(&new_path, &content)?;
            *path = Some(new_path);
            println!("File saved successfully");
        } else {
            println!("Save operation canceled");
        }
    }

    Ok(())
}