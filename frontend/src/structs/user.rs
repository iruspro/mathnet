
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

#[derive(Debug,Clone)]
pub enum UserDemandsUserProfileDataChanges{
    ChangeUserName(String,UserChangingProfileData),
    ChangeUserEmail(String, UserChangingProfileData),
    ChangeUserPassword(String, UserChangingProfileData),
    ChangeUserPasswordConfirmation(String, UserChangingProfileData),
    ConfirmChanges(UserChangingProfileData),
}

#[derive(Debug,Clone)]
pub struct UserChangingProfileData<'a>{
    pub user_id : u32,
    pub new_user_name : String,
    pub new_user_password : String,
    pub new_user_password_confirmation : String,
    pub new_user_email: String

}

impl<'a> UserChangingProfileData<'a>{
    pub fn new() -> UserChangingProfileData<'a>{
        UserChangingProfileData{
        user_id : 42,
        new_user_name : String::new(),
        new_user_email : String::new(),
        new_user_password : String::new(),
        new_user_password_confirmation : String::new(),
        }
    }
    pub fn change_user_name(&mut self, selected_user_name : String){
    self.new_user_name = selected_user_name;
        }
    
    pub fn change_user_email(&mut self, selected_user_email : String){
        self.new_user_email = selected_user_email;
    }
    pub fn change_user_password(&mut self, selected_user_password : String){
        self.new_user_password = selected_user_password;
    }

    pub fn change_user_password_confirmation(&mut self, selected_user_password : String){
        self.new_user_password = selected_user_password;
    }
    pub fn confirm_changes(self) {
        
    }}
