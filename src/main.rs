use std::rc::Rc;
use tokio::time::{Duration};
use futures::channel::mpsc;
use futures::SinkExt;
use slint::{Model, SharedString};
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
    ui.global::<MenuCallback>().on_undo(move || {
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

    let text_history = app_state.get_text_history().clone();
    let ui_handle = ui.as_weak();
    let is_programmatic_change = app_state.get_is_programmatic_change();
    ui.global::<MenuCallback>().on_redo(move || {
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
    ui.global::<TabManager>().on_new_tab(move || {
        let file_name = "file.txt";
        eprintln!("new tab created");
        let ui = ui_handle.unwrap();
        let mut tabs: Vec<SharedString>= ui.get_tab_titles().iter().collect();
        tabs.push(file_name.into());
        let final_tab = Rc::new(slint::VecModel::from(tabs.clone()));
        ui.set_tab_titles(final_tab.into());
        println!("{:?}",tabs);
    });

    //let ui_handle = ui.as_weak();
    let text_history = app_state.get_text_history();
    let current_file_path = app_state.get_current_file_path();
    ui.global::<MenuCallback>().on_save(move ||{
        let history = text_history.lock().unwrap();
        let content = history.history[history.current_index].clone();
        if let Err(e) = utils::save_file(&content, &current_file_path){
            eprintln!("Error saving file {}", e);
        }
    });

    let ui_handle = ui.as_weak();
    let current_file_path = app_state.get_current_file_path();
    ui.global::<MenuCallback>().on_save_as(move || {
        let ui = ui_handle.unwrap();
        // Assuming you have a way to get the current
        let content = ui.get_content();
        if let Err(e) = utils::save_as(&content, &current_file_path) {
            eprintln!("Error saving file: {}", e);
        }
    });

    let ui_handle = ui.as_weak();
    let text_history = app_state.get_text_history();
    let file_path = app_state.get_current_file_path();
    let is_programmatic_change = app_state.get_is_programmatic_change();
    ui.global::<MenuCallback>().on_open(move ||{
        let ui = ui_handle.unwrap();
        if let Ok(content) = utils::open_file(&file_path){
            let mut history = text_history.lock().unwrap();
            history.add_change(content.clone());

            *is_programmatic_change.lock().unwrap() = true;
            ui.set_content(content.into());
            *is_programmatic_change.lock().unwrap() = false;

            ui.set_undo_enabled(history.can_undo());
            ui.set_redo_enabled(history.can_redo());
        } else {
            eprintln!("Error opening file ");
        }
    });

    ui.run()?;
    Ok(())
}
