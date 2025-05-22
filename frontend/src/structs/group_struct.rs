pub use crate::structs::chat_message;
pub use crate::structs::user::*;

#[derive(Debug, Clone)]

pub enum GroupId {
    GroupId(u32),
}

#[derive(Debug, Clone)]
pub struct Group {
    pub group_id: GroupId,
    pub group_members: Vec<UserId>,
    pub number_of_members: u32,
}

pub struct Channel {
    pub group_id: GroupId,
    pub list_of_channel_messages: Vec<chat_message::ChatMessage>,
    pub list_of_members: Vec<UserId>,
}
