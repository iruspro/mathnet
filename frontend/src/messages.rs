use crate::structs::user::{UserLoginData, UserRegisterData, get_user_register_data};


#[derive(Debug,Clone)]
pub enum Msg {
    SetPage(GoToPage),
    LoginAttempt(UserLoginAttempt),
    Registration(UserRegister),
    UserWantsToChangeProfileData(UserChangingProfileData),
}

#[derive(Debug,Clone)]
pub enum GoToPage{
GoToPageSigned(SwitchToPageSigned),
GoToPageUnsigned(SwitchToPageUnsigned)
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

#[derive(Debug,Clone)]
pub enum UserChangingProfileData{
    UserEmailChanged(String),

}