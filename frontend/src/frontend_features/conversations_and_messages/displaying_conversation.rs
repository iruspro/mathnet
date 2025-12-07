/*
Displaying conversation, i. e. display conversation messages in some nice fashion.
*/


pub use sauron::Node;
use crate::experimental_examples::imaginary_chat_messages::dummy_chat_messages;
pub use crate::messages::*;

pub use crate::structs::user::{User,UserId,new};
pub use crate::frontend_features::conversations_and_messages::{conversation_message::ConversationMessage, post_new_message::post_new_message};
pub use crate::structs::friends_at_sidebar::{FriendAtSidebar,ShowChats};
pub use sauron::{node,text};
pub use crate::app::App;
pub use crate::communication_with_server::get_conversation::get_whole_conversation_from_server;
pub use crate::list_of_pages::Page;



pub fn show_conversation(conversation_id : u128, current_page: &Page ) -> Node<Msg>{
    let tuple_of_vec_of_chat_messages_and_friend_name = get_whole_conversation_from_server(conversation_id);
    let friend_name:String = tuple_of_vec_of_chat_messages_and_friend_name.1;
    let vec_of_chat_messages_unused:Vec<ConversationMessage> = tuple_of_vec_of_chat_messages_and_friend_name.0;
    let vec_of_chat_messages = dummy_chat_messages();
    
    node! {
            <div class="mycontent">
                <div class="container-fluid"> 
                    <h1 class="text-center">{text!("{}",friend_name)}</h1>
<div class="d-grid gap-2 d-md-flex justify-content-md-start">
  <button class="btn btn-primary" type="button" on_click=|_| {Msg::SetConversationToNone}>"Go back to all messages"</button>
  </div>
                        {for conversation_message in vec_of_chat_messages{
                            conversation_message.conversation_message_display()
                            
                        }}
                </div>
{post_new_message()}
            </div>
    }}
