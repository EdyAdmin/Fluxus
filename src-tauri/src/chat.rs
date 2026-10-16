// File dedicated to the chats.

// Import for JSON conversion.
use serde::{Deserialize, Serialize};

// Import for UUIDV7.
use uuid::Uuid;

// Import for time functions.
use std::time::{SystemTime, UNIX_EPOCH};

// Defined the enumeration Role the different roles that can send a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    User,
    Assistant,
    Manager,
    System,
}

// Defined the struct Message, that defines a standard message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub timestamp: u64,
    pub id: Uuid,
    pub role: Role,
    pub content: String,
    pub model: Option<String>,
}

// Defined the struct Conversation, a series of messages. Each conversation has a manager, that coordinates the messages.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Conversation {
    pub manager: Option<String>,
    pub theme: String,
    pub messages: Vec<Message>,
}

// The implementation of Message.
impl Message {
    // Default constructor.
    pub fn new(role: Role, content: String, model: Option<String>) -> Self {
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        Self {
            timestamp,
            id: Uuid::now_v7(),
            role,
            content,
            model,
        }
    }

    // Constrcutror for user messages.
    pub fn user_message(content: String) -> Self {
        Self::new(Role::User, content, None)
    }
}

// The implementation of Conversation.
impl Conversation {
    // Default constructor.
    pub fn new() -> Self {
        Self::default()
    }

    // Function for adding messages to a conversation.
    pub fn add_message(&mut self, msg: Message) {
        self.messages.push(msg);
    }
}
