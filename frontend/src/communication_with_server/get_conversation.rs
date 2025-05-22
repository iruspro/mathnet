use sauron::web_sys::console;

use crate::experimental_examples::imaginary_chat_messages::dummy_chat_messages;
pub use crate::structs::chat_message::{ChatId, ChatMessage};

pub fn get_whole_conversation_from_server(chat_id: ChatId) -> (Vec<ChatMessage>, String) {
    //String is for getting your friend's name
    console::log_1(&"Rust6".into());
    (dummy_chat_messages(), "John Doe".to_string())
}
