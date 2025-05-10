pub use sauron::Node;
pub use crate::messages::Msg;

pub use crate::structs::user::{User,UserId,new};
pub use crate::structs::chat_message::{ChatId,ChatMessage};
pub use crate::structs::friends_at_sidebar::{FriendAtSidebar,ShowChats};
pub use sauron::{node,text};
pub use crate::app::App;

/* When you click on some person in show_chats_in_content, you get the selected conversation.*/
pub fn show_conversation() -> Node<Msg>{
    unimplemented!()
} 