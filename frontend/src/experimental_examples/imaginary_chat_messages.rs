pub use crate::frontend_features::conversations_and_messages::conversation_message::ConversationMessage;

pub fn dummy_chat_messages()-> Vec<ConversationMessage>{
    let mut vec_of_messages:Vec<ConversationMessage> = Vec::new();
for _ in 0..100{
    vec_of_messages.push(ConversationMessage::new());
}
vec_of_messages
}