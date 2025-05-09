pub use crate::structs::user;
pub use crate::structs::emojis_and_reactions;

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

pub struct ChatMessage{
    chat_id: ChatId,
    chat_message_id : ChatMessagesId,
    date_and_time : (),
    content: String,
    latex_on : bool,
    sender: user::UserId,
    reactions : Vec<emojis_and_reactions::Emoji>,

}