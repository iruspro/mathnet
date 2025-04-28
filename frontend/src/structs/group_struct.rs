pub use crate::structs::user::*;

#[derive(Debug,Clone)]

pub enum GroupId{
    GroupId(u32),
}

#[derive(Debug,Clone)]
pub struct Group{
    pub group_id : GroupId,
    pub group_members : Vec<UserId>,
    pub number_of_members : u32,
}

