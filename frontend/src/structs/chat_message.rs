pub use crate::structs::user;
pub use crate::structs::emojis_and_reactions;
pub use crate::logics::date_and_time::{DateAndTime, display_current_date_and_time};

#[derive(Clone,Debug)]
pub enum ChatId{
    ChatIdNumber(u32),
}

pub struct ChatChannel{
    chat_id : ChatId,
    chat_messages_vec : Vec<ChatMessage>,
}

pub enum ChatMessagesId{
    ChatMessagesIdValue(u32),
}

/* ChatMessage represents a message that some person can 
send to other person via personal conversation */
pub struct ChatMessage{
    pub chat_id: ChatId,
    pub chat_message_id : ChatMessagesId,
    pub date_and_time : DateAndTime,
    pub content: String,
    pub latex_on : bool,
    pub sender: user::UserId,
    pub reactions : Vec<emojis_and_reactions::Emoji>,

}

// Implement that!
impl ChatMessage{
    pub fn new() -> ChatMessage{
        ChatMessage { chat_id: ChatId::ChatIdNumber(111), chat_message_id: ChatMessagesId::ChatMessagesIdValue(112), date_and_time: display_current_date_and_time(), content: "Lorem ipsum".to_string(), latex_on: false, sender: user::UserId::UserIdNumber(42), reactions: Vec::new()}
    }
}