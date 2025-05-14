use crate::structs::user::{UserLoginData, UserRegisterData, get_user_register_data,UserChangingProfileData,UserDemandsUserProfileDataChanges,UserId};
use crate::structs::chat_message::ChatId;

#[derive(Debug,Clone)]
pub enum Msg {
    SetPage(GoToPage),
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
pub enum GoToPage{
GoToPageSigned(SwitchToPageSigned),
GoToPageUnsigned(SwitchToPageUnsigned),
GoToPageOther(SwitchToPageOther),
}

#[derive(Debug,Clone)]
pub enum SwitchToPageUnsigned {
    GoToHomePage,
    GoToLoginPage,
    GoToDocsPage,
    GoToRegister,
    GoToAboutProject,
    GoToPrivacyAndSecurity,
}

#[derive(Debug,Clone)]
pub enum SwitchToPageSigned {
    GoToDocsPage,
    GoToAboutProject,
    GoToPrivacyAndSecurity,
    GoToGroupsList,
    GoToUserProfile,
    GoToSettings,
    GoToLogOut,
    GoToChatWithFriends,
    GoToNotifications,
}

#[derive(Debug,Clone)]
pub enum SwitchToPageOther{
    SuccessfullyChangedUserProfileData,
    GoToDeleteAccount,
    GoToRetryChangingUserProfileData,
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

