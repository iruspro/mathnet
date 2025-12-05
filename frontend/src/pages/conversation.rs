/*
Conversation with individual friends or groups.
*/

use sauron::prelude::*;
use crate::logics::displaying_conversation::show_conversation;
use crate::logics::post_new_message::post_new_message;
use crate::messages::{Msg};
use crate::app::App;
use crate::logics::{displaying_friends::{show_friends_at_sidebar,show_chats_in_content}, displaying_conversation, post_new_message};
use crate::experimental_examples::imaginary_friends;
use crate::web_sys::console;

pub fn conversation_display(data_provided : &App) -> Node<Msg> {
console::log_1(&"Rust7".into());
show_conversation(data_provided.current_conversation.clone().unwrap(), &data_provided.current_page.clone())
}