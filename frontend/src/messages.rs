use crate::structs::chat_message::ChatId;
use crate::structs::user::{
    UserChangingProfileData, UserDemandsUserProfileDataChanges, UserId, UserLoginData,
    UserRegisterData, get_user_register_data};
use crate::list_of_pages::Page;

#[derive(Debug, Clone,PartialEq)]
pub enum DefinedMsg {
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

/* This enums are not needed anymore, because there is simpler way to 
do page swithcing using only SetPage message

#[derive(Debug, Clone)]
pub enum GoToPage {
    GoToPageSigned(SwitchToPageSigned),
    GoToPageUnsigned(SwitchToPageUnsigned),
    GoToPageOther(SwitchToPageOther),
}

#[derive(Debug, Clone)]
pub enum SwitchToPageUnsigned {
    GoToHomePage,
    GoToLoginPage,
    GoToDocsPage,
    GoToRegister,
    GoToAboutProject,
    GoToPrivacyAndSecurity,
}

#[derive(Debug, Clone)]
pub enum SwitchToPageSigned {
    GoToDocsPage,
    GoToAboutProject,
    GoToPrivacyAndSecurity,
    GoToGroupsList,
    GoToUserProfile,
    GoToSettings,
    GoToChatWithFriends,
    GoToNotifications,
}

#[derive(Debug, Clone)]
pub enum SwitchToPageOther {
    GoToSuccessfullyChangedUserProfileData,
    GoToDeleteAccount,
    GoToRetryChangingUserProfileData,
    GoToLogOut,
}

*/

#[derive(Debug, Clone,PartialEq)]
pub enum UserLoginAttempt {
    UpdateUserName(String),
    UpdateUserPassword(String),
    CheckLoginValidy,
}

#[derive(Debug, Clone,PartialEq)]
pub enum UserRegister {
    UpdateUserRegisterName(String),
    UpdateUserRegisterPassword(String),
    UpdateUserRegisterPasswordAgain(String),
    UpdateUserRegisterEmail(String),
    RegistrationAttempt,
}
