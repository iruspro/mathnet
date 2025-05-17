use sauron::prelude::*;
use crate::logics::displaying_conversation::show_conversation;
use crate::messages::{Msg, GoToPage, SwitchToPageSigned};
use crate::app::App;
use crate::logics::{displaying_friends::{show_friends_at_sidebar,show_chats_in_content}, displaying_conversation};
use crate::experimental_examples::imaginary_friends;
use crate::web_sys::console;

pub fn view(data_provided : &App) -> Node<Msg> {
console::log_1(&"Rust7".into());
show_conversation(data_provided.current_conversation.clone().unwrap())
}