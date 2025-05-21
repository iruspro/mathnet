use crate::app::App;
use crate::experimental_examples::imaginary_friends;
use crate::logics::displaying_conversation::show_conversation;
use crate::logics::post_new_message::post_new_message;
use crate::logics::{
    displaying_conversation,
    displaying_friends::{show_chats_in_content, show_friends_at_sidebar},
    post_new_message,
};
use crate::messages::{GoToPage, Msg, SwitchToPageSigned};
use crate::web_sys::console;
use sauron::prelude::*;

pub fn view(data_provided: &App) -> Node<Msg> {
    console::log_1(&"Rust7".into());
    show_conversation(
        data_provided.current_conversation.clone().unwrap(),
        data_provided.current_page.clone(),
    )
}
