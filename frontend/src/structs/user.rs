
pub use crate::structs::group_struct::GroupId;
pub use crate::structs::languages;

use self::languages::Languages;
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

#[derive(Debug,Clone)]
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
    pub language : Languages,
}

pub fn new() -> User {
    User { user_id: UserId::UserIdNumber(1), user_name: "John Doe".to_string(), user_password: "MathNetIsTheBest!".to_string(), user_email: "john.doe@gmail.com".to_string(), user_friends: vec![], user_groups: vec![], language: Languages::English}
}