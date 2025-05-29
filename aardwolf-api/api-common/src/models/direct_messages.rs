// aardwolf-api-common/src/models/direct_messages.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PrivateMessage {
    pub id: i32,
    pub sender_id: i32,
    pub recipient_id: i32,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct PrivateMessageReply(pub PrivateMessage);
