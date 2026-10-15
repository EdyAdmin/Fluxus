// File dedicated to the implementation of the app.

// Import of chats.
pub mod chat;
use chat::{Message, Conversation, Role};

// Import for mutexes and shared memory.
use std::sync::Mutex;
use tauri::State;


// Function for sending a message.
#[tauri::command]
fn send_message(content: String, state: State<'_, Mutex<Conversation>>) -> Message {
    let mut conv: std::sync::MutexGuard<'_, Conversation> = state.lock().unwrap();
    let message = Message::user_message(content);
    conv.add_message(message.clone());
    println!("El historial de mensajes es: {:?}", conv.messages);
    message
}
// Function the default run.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(Conversation::new()))
        .invoke_handler(tauri::generate_handler![send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
