use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation{
    pub messages: Vec<Message>
}

impl Conversation{
    pub fn new() -> Conversation {
        Conversation {
            messages: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message{
    pub user: bool, // is user or is chat-bot
    pub text: String,
}
