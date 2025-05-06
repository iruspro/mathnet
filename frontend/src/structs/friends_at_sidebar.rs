pub use sauron::Node;
pub use crate::messages::Msg;

pub use crate::structs::user::{User,UserId};
pub use sauron::node;
pub use crate::app::App;


pub struct FriendAtSidebar{
    friend : UserId,
    ordering_number : u32,
}

pub fn get_user_friends_data_from_server(vec_of_friends_id : &Vec<FriendAtSidebar>) -> Vec<User>{
unimplemented!("This piece of code will be implemented when I figure out how to manage 
communication with the server.")
}
pub fn show_friends_at_sidebar(current_state_of_app: &App) -> Node<Msg>{
    let friends_data = get_user_friends_data_from_server(&current_state_of_app.user_data.user_friends);
    let iterator_of_friends = friends_data.iter();
node!{
{for friend in iterator_of_friends{
    node!{
        <div class="card">
        <div class="card-body">
        <a on_click = |_|{Msg::UserWantsToChatWithSomePerson(friend.user_id.clone())}>"some_dummy_text"</a>
        </div>
        </div>
    }
}
}}}

