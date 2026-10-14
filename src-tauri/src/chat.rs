// Clase dedicada a los chats.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role{
    User,
    Assistant,
    Manager,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message{
    pub role: Role,
    pub content: String,
    pub model_used: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Conversation{
    pub gestor: Option<String>,
    pub theme: String,
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_message(&mut self, msg: Message) {
        self.messages.push(msg);
    }
}