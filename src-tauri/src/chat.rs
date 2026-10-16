// File dedicated to the chats.

// Import for JSON conversion.
use serde::{Deserialize, Serialize};

// Import for some concurrency functions.
use std::sync::atomic::{AtomicU64, Ordering};

// Import for time functions.
use std::time::{SystemTime, UNIX_EPOCH};

static COUNTER: AtomicU64 = AtomicU64::new(1);

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
    pub timestamp: u64,
    pub id: u64,
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

fn time_millis_and_id() -> (u64,u64) {
    // Generates the real time.
    let time: std::time::Duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    // Generates the timestamp.
    let timestamp: u64 = time.as_secs() * 1000 + (time.subsec_millis() as u64);
    let nanos: u64 = time.subsec_nanos() as u64;
    let count: u64 = COUNTER.fetch_add(1, Ordering::Relaxed);
    // Generates the id.
    let random: u64 = ((nanos^count).wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 48) & 0xFFFF;
    let id: u64 = (timestamp & 0xFFFF_FFFF_FFFF) << 16 | random;
    (timestamp, id)
}

// The implementation of Message.
impl Message {
    // Default constructor.
    pub fn new(role: Role, content: String, model: Option<String>) -> Self {
        let (timestamp, id): (u64, u64) = time_millis_and_id();
        Self {
            timestamp,
            id,
            role, 
            content, 
            model }
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