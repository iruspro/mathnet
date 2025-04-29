
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
    ChangeUserName(String,&mut UserChangingProfileData),
    ChangeUserEmail(String, &mut UserChangingProfileData),
    ChangeUserPassword(String, &mut UserChangingProfileData),
    ConfirmChanges(UserChangingProfileData),
}

#[derive(Debug,Clone)]
pub struct UserChangingProfileData{
    pub new_user_name : Option<String>,
    pub new_user_password : Option<String>,
    pub new_user_email: Option<String>

}

impl UserChangingProfileData{
    pub fn new() -> UserChangingProfileData{
        UserChangingProfileData{
        new_user_name : None,
        new_user_email : None,
        new_user_password : None,
        }
    }
    pub fn change_user_name(&mut self, selected_user_name : String){
        match selected_user_name {
            None => self.new_user_name = None,
            Some(new_name) => self.new_user_name = Some(new_name),
        }
    }
    pub fn change_user_email(&mut self, selected_user_email : Option<String>){
        match selected_user_email {
            None => self.new_user_email = None,
            Some(new_email) => self.new_user_email = Some(new_email),
        }
    }
    pub fn change_user_password(&mut self, selected_user_password : Option<String>){
        match selected_user_password {
            None => self.new_user_password = None,
            Some(new_password) => self.new_user_password = Some(new_password),
        }
    }
    pub fn sent_user_profile_data(self) {
        
    }
}