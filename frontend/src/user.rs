
pub use crate::group_struct::GroupId;
#[derive(Debug,Clone)]
pub struct UserLoginData{
    pub user_name : String,
    pub user_password : String,
}

pub struct UserRegisterData{
    pub user_name : String,
    pub user_password : String,
    pub user_password_again : String,
    pub user_email : String
}

pub fn get_user_register_data() -> UserRegisterData {
    UserRegisterData { user_name: String::new(), user_password: String::new(), user_password_again: String::new(), user_email: String::new()}
}

pub enum UserId{
UserIdNumber(u32),
}
pub struct User{
    pub user_id : UserId,
    pub user_name : String,
    pub user_password : String,
    pub user_email: String,
    pub user_friends : Vec<UserId>,
    pub user_groups: Vec<GroupId>,
}