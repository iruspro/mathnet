pub use sauron::Node;
pub use crate::messages::Msg;

pub use crate::structs::user::{User,UserId,new};
pub use crate::structs::chat_message::{ChatId,ChatMessage};
pub use sauron::node;
pub use crate::app::App;
pub use crate::logics::date_and_time;

use self::date_and_time::display_current_date_and_time;

use super::chat_message::DateAndTime;

#[derive(Debug,Clone)]
pub struct FriendAtSidebar{
    pub friend : UserId,
    pub ordering_number : u32,
}


/*Struct ShowChats represents a chat channel in chat_with_friends.rs default view */
pub struct ShowChats{
    pub chat_id : ChatId, 
    pub friend : User,
    pub last_message : ChatMessage,
    pub date_and_time : DateAndTime,
    pub currently_active: bool,
}


impl ShowChats{
    pub fn new() -> ShowChats{
        ShowChats{
        chat_id : ChatId::ChatIdNumber(111),
        friend : new(),
        last_message : ChatMessage::new(),
        date_and_time : display_current_date_and_time(),
        currently_active : false,
        } 
    }
}
