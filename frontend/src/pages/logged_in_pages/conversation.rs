use sauron::prelude::*;
use crate::logics::displaying_conversation::show_conversation;
use crate::messages::{Msg, GoToPage, SwitchToPageSigned};
use crate::app::App;
use crate::logics::{displaying_friends::{show_friends_at_sidebar,show_chats_in_content}, displaying_conversation};
use crate::experimental_examples::imaginary_friends;

pub fn view(data_provided : &App) -> Node<Msg> {
show_conversation(data_provided.current_conversation.clone().unwrap())
}