/*
List of conversations with friends with latest message displayed at each conversation summary.
Additional features are find friends, add friend, remove friend, friend requests ...
*/

use sauron::prelude::*;
use crate::app::Msg;
use crate::experimental_examples::imaginary_friends;
use crate::frontend_features::conversations_and_messages::conversation_summaries::dummy_conversation_summary;

pub fn conversation_list_display() -> Node<Msg> {
node!{<h1 class="text-center">"Find friends"</h1>
                    <p class="basicparagraph text-start">
                        "View other people's profiles or chat with somebody and have fun!"
                        for _ in (0..100){dummy_conversation_summary()
                        }
                    </p>}
}