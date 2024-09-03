use std::sync::{Arc, Mutex};
use tokio::time::{Duration, Instant};
use futures::channel::mpsc;
use futures::SinkExt;
use slint::SharedString;
use crate::lib::texthistory::TextHistory;
use crate::ui::rxeditor_view::AppState;
use crate::ui::utils;
slint::include_modules!();


mod lib;
mod ui;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let ui = RxTextEdittor::new()?;

    let app_state = AppState::new();

    let (sender, receiver) = mpsc::channel(100);

    let sender_clone = sender.clone();
    let is_programmatic_change = app_state.get_is_programmatic_change();
    let ui_handle = ui.as_weak();
    let ui_handle2 = ui.as_weak();
    let text_history = app_state.get_text_history();
    ui.global::<TextContent>().on_text_editted(move |text: SharedString| {
        if !*is_programmatic_change.lock().unwrap(){
            let mut sender = sender_clone.clone();
            let ui_handle = ui_handle.clone();
            let text_history = text_history.clone();
            tokio::spawn(async move {
                if let Err(e) = sender.send(text.to_string()).await {
                    eprintln!("Failed to send text: {}", e);
                }
                // Immediately enable undo button on first change
                if let Some(ui) = ui_handle.upgrade() {
                    let history = text_history.lock().unwrap();
                    println!("{}", history.can_undo());
                    ui.set_undo_enabled(history.can_undo());
                }
            });
            let ui_handle = ui_handle2.clone();
            let ui = ui_handle.upgrade().unwrap();
            ui.set_undo_enabled(true);
            ui.set_redo_enabled(false);
        }
    });

    let text_history_clone = app_state.get_text_history();
    let ui_handle = ui.as_weak();
    let is_programmatic_change_clone = app_state.get_is_programmatic_change();
    tokio::spawn(utils::debouncer(receiver, Duration::from_millis(0),is_programmatic_change_clone ,text_history_clone, ui_handle));

    let text_history = app_state.get_text_history();
    let ui_handle = ui.as_weak();
    let is_programmatic_change = app_state.get_is_programmatic_change();
    ui.global::<MyMenuCallback>().on_undo(move || {
        let mut history = text_history.lock().unwrap();
        let ui = ui_handle.unwrap();
        if let Some(text) = history.undo() {
            *is_programmatic_change.lock().unwrap() = true;
            ui.set_content(text.into());
            *is_programmatic_change.lock().unwrap() = false;
            ui.set_undo_enabled(history.can_undo());
            ui.set_redo_enabled(history.can_redo());
        }
    });

    let text_history = app_state.text_history.clone();
    let ui_handle = ui.as_weak();
    let is_programmatic_change = app_state.get_is_programmatic_change();
    ui.global::<MyMenuCallback>().on_redo(move || {
        let mut history = text_history.lock().unwrap();
        let ui = ui_handle.unwrap();
        if let Some(text) = history.redo() {
            *is_programmatic_change.lock().unwrap() = true;
            ui.set_content(text.into());
            *is_programmatic_change.lock().unwrap() = false;
            ui.set_undo_enabled(history.can_undo());
            ui.set_redo_enabled(history.can_redo());
        }
    });

    let ui_handle = ui.as_weak();
    ui.global::<MyMenuCallback>().on_save_as(move || {
        let ui = ui_handle.unwrap();
        // Assuming you have a way to get the current
        let content = ui.get_content();
        if let Err(e) = utils::save_as(&content) {
            eprintln!("Error saving file: {}", e);
        }
    });

    ui.run()?;
    Ok(())
}
