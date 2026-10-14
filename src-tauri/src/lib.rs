// File dedicated to the implementation of the app.

// Import of chats.
pub mod chat;
use chat::{Conversation, Role};

// Import for mutexes and shared memory.
use std::sync::Mutex;
use tauri::State;

// Function the default run.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
