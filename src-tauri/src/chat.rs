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
    pub model: Option<String>,
}

// Defined the struct Conversation, a series of messages. Each conversation has a manager, that coordinates the messages.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Conversation{
    pub manager: Option<String>,
    pub theme: String,
    pub messages: Vec<Message>,
}

// The implementation of Message.
impl Message {
    // Default constructor.
    pub fn new(role: Role, content: String, model: Option<String>) -> Self {
        Self { role, content, model }
    }

    // Constrcutror for user messages.
    pub fn user_message(content: String) -> Self {
        Self {
            role: Role::User,
            content,
            model: None,
        }
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