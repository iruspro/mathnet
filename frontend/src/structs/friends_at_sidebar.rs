pub use sauron::Node;
pub use crate::messages::Msg;

pub use crate::structs::user::{User};

pub struct FriendAtSidebar{
    friend : User,
    ordering_number : u32,

}


// pub fn show_friends_at_sidebar(vector_of_friends: & Vec<FriendAtSidebar>) -> Node<Msg>{
// let iterator_of_friends = vector_of_friends.iter();
// for friend in vector_of_friends{
//     node!{
//         <p>Msg::NoAction</p>
//     }
// 
// }
// }

