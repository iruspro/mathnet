use crate::structs::user::{UserLoginData, UserRegisterData, get_user_register_data,UserChangingProfileData,UserDemandsUserProfileDataChanges,UserId};
use crate::structs::chat_message::ChatId;
use crate::list_of_pages::Page;

#[derive(Debug,Clone)]
pub enum Msg {
    SetPage(Page),
    LoginAttempt(UserLoginAttempt),
    Registration(UserRegister),
    UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges),
    SearchFriend(String),
    NoAction,
    UserWantsToChatWithSomePerson(UserId),
    UserWantsToChatWithSomePersonViaPersonalConversation(ChatId),
    SetConversationToNone,
    NoOp,
    SendConversationMessage(String),
}





#[derive(Debug,Clone)]
pub enum UserLoginAttempt {
    UpdateUserName(String),
    UpdateUserPassword(String),
    CheckLoginValidy,
}

#[derive(Debug,Clone)]
pub enum UserRegister {
    	UpdateUserRegisterName(String),
        UpdateUserRegisterPassword(String),
        UpdateUserRegisterPasswordAgain(String),
        UpdateUserRegisterEmail(String),
        RegistrationAttempt
}

