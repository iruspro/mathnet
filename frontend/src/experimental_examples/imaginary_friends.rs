use crate::structs::chat_message::ChatChannel;
pub use crate::structs::friends_at_sidebar::*;
use crate::structs::user;

use self::date_and_time::display_current_date_and_time;
// Implement that to test if displaying messages at chat_with_friends_work properly - try using actual tests

pub fn dummy_user() -> User {
    user::User {
        user_id: user::UserId::UserIdNumber(123),
        user_name: "John Doe".to_string(),
        user_password: "EasyPassword".to_string(),
        user_email: "john.doe@gmail.com".to_string(),
        user_friends: Vec::new(),
        user_groups: Vec::new(),
        language: user::languages::Languages::English,
    }
}

pub fn dummy_show_chat() -> ShowChats {
    ShowChats {
        chat_id: ChatId::ChatIdNumber(111),
        friend: dummy_user(),
        last_message: ChatMessage::new(),
        date_and_time: display_current_date_and_time(),
        currently_active: true,
    }
}
