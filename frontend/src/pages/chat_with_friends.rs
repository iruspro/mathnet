/*
List of chats with friends with latest message displayed at each chat.
*/

use sauron::prelude::*;
use crate::app::Msg;
use crate::logics::{displaying_friends::{show_friends_at_sidebar,show_chats_in_content}, displaying_conversation};
use crate::experimental_examples::imaginary_friends;

fn chat_with_friends_display() -> Node<Msg> {
node!{<h1 class="text-center">"Find friends"</h1>
                    <p class="basicparagraph text-start">
                        "View other people's profiles or chat with somebody and have fun!"
                        {for _ in (0..100){show_chats_in_content(vec!(
                            imaginary_friends::dummy_show_chat(),
                            ))
                        }}
                    </p>}
}