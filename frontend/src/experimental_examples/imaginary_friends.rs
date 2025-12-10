

use crate::structs::user::{User, UserId};
use crate::structs::languages::Languages;

use crate::frontend_features::date_and_time::date_and_time::display_current_date_and_time;
// Implement that to test if displaying messages at chat_with_friends_work properly - try using actual tests


pub fn dummy_user() -> User{
User{
    user_id : UserId::UserIdNumber(123),
    user_name : "John Doe".to_string(),
    user_password: "EasyPassword".to_string(),
    user_email : "john.doe@gmail.com".to_string(),
    language : Languages::English,
}}

/*
pub fn dummy_show_chat()-> ShowChats{
ShowChats{
    chat_id : ChatId::ChatIdNumber(111),
    friend : dummy_user(),
    last_message : ChatMessage::new(),
    date_and_time : display_current_date_and_time(),
    currently_active : true
}}
*/

