use sauron::web_sys::console;

use crate::experimental_examples::imaginary_chat_messages::dummy_chat_messages;
use crate::frontend_features::conversations_and_messages::conversation_message::ConversationMessage;


pub fn get_whole_conversation_from_server(conversation_id : u128) -> (Vec<ConversationMessage>, String){ //String is for getting your friend's name
    console::log_1(&"Rust6".into());
    (dummy_chat_messages(),"John Doe".to_string())
    
}