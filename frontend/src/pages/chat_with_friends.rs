use sauron::prelude::*;
use crate::app::Msg;
fn chat_with:friends_content() -> Node<Msg> {
node!{<h1 class="text-center">"Find friends"</h1>
                    <p class="basicparagraph text-start">
                        "View other people's profiles or chat with somebody and have fun!"
                        {for _ in (0..100){show_chats_in_content(vec!(
                            imaginary_friends::dummy_show_chat(),
                            ))
                        }}
                    </p>}
}