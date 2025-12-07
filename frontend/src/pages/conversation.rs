/*
Conversation with individual friends or groups.
*/

use sauron::prelude::*;
use crate::messages::{Msg};
use crate::app::App;
use crate::experimental_examples::imaginary_friends;
use crate::web_sys::console;
use crate::frontend_features::conversations_and_messages::displaying_conversation::show_conversation;


pub fn conversation_display(data_provided : &App) -> Node<Msg> {
console::log_1(&"Rust7".into());
show_conversation(data_provided.current_conversation.clone().unwrap(), &data_provided.current_page.clone())
}