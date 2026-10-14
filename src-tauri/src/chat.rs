// File dedicated to the chats.

// Import for JSON conversion.
use serde::{Deserialize, Serialize};

// Defined the enumeration Role the different roles that can send a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role{
    User,
    Assistant,
    Manager,
    System,
}

// Defined the struct Message, that defines a standard message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message{
    pub role: Role,
    pub content: String,
    pub model_used: Option<String>,
}

// Defined the struct Conversation, a series of messages. Each conversation has a manager, that coordinates the messages.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Conversation{
    pub manager: Option<String>,
    pub theme: String,
    pub messages: Vec<Message>,
}

// The implementation of Conversation. 
impl Conversation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_message(&mut self, msg: Message) {
        self.messages.push(msg);
    }
}