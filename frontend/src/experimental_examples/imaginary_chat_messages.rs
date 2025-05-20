pub use crate::structs::chat_message;

pub fn dummy_chat_messages()-> Vec<chat_message::ChatMessage>{
    let mut vec_of_messages:Vec<chat_message::ChatMessage> = Vec::new();
for _ in 0..100{
    vec_of_messages.push(chat_message::ChatMessage::new());
}
vec_of_messages
}