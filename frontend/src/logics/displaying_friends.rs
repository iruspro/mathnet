pub use sauron::Node;
pub use crate::messages::Msg;

pub use crate::structs::user::{User,UserId,new};
pub use crate::structs::chat_message::{ChatId,ChatMessage};
pub use crate::structs::friends_at_sidebar::{FriendAtSidebar,ShowChats};
pub use sauron::{node,text};
pub use crate::app::App;


/*TODO: migrate this functions to logics map to some new rs file. */
pub fn get_user_friends_data_from_server(vec_of_friends_id : Vec<FriendAtSidebar>) -> Vec<User>{
vec![new()]

 /*   unimplemented!("This piece of code will be implemented when I figure out how to manage 
communication with the server.") */
}

// Current function show_friends_at_sidebar clones data, but I don't think that is necessary ... or is it? Think about it again.
pub fn show_friends_at_sidebar(current_state_of_app: &App) -> Node<Msg>{
    let friends_data = get_user_friends_data_from_server(current_state_of_app.user_data.user_friends.clone());
node!{
{for friend in friends_data{
    node!{
        <div class="card">
        <div class="card-body">
        <a on_click =move |_| {Msg::UserWantsToChatWithSomePerson(friend.user_id.clone())}>text(friend.user_name.clone())</a>
        </div>
        </div>
    }
}
}}}

pub fn show_chats_in_content(v : Vec<ShowChats>) -> Node<Msg>{
    node!{
    {for chat in v{
        node!{
            <div class="card">
            <div class="card-body">
            <a on_click =move |_| {Msg::UserWantsToChatWithSomePersonViaPersonalConversation(chat.chat_id.clone())}>{text!("{}",chat.friend.user_name.clone())}</a>
            <p>{text!("{}", chat.last_message.content.clone())}</p>
            <p>{text!("{}",chat.date_and_time.date_and_time_to_string())}</p>
            <p></p>
            </div>
            </div>
        }}
    }
    }
}