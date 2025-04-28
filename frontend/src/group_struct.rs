pub use crate::user::*;

#[derive(Debug,Clone)]

pub enum GroupId{
    GroupId(u32),
}
pub struct Group{
    pub group_id : GroupId,
    pub group_members : Vec<UserId>,
}