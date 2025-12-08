use crate::structs::user::{UserLoginData, UserRegisterData, get_user_register_data,UserChangingProfileData,UserDemandsUserProfileDataChanges,UserId};
use crate::list_of_pages::Page;

#[derive(Debug,Clone)]
pub enum Msg {
    SetPage(Page),
    LoginAttempt(UserLoginAttempt),
    Registration(UserRegister),
    UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges),
    SearchFriend,
    NoAction,
    UserWantsToChatWithSomePerson(UserId),
    UserWantsToChatWithSomePersonViaPersonalConversation(u128),
    SetConversationToNone,
    NoOp,
    SendConversationMessage(String),
    DeleteAccount,
    UpdatePersonSearch(String),
    SendFriendRequest,
    UnsendFriendRequest,
    UpdateFriendSearch(String),
    UnblockFriend,
    BlockFriend,
    RemoveFriend

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

