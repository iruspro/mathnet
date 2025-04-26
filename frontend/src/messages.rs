use crate::user::{UserLoginData, UserRegisterData, get_user_register_data};


#[derive(Debug,Clone)]
pub enum Msg {
    SetPage(GoToPage),
    LoginAttempt(UserLoginAttempt),
    Registration(UserRegister)
}

#[derive(Debug,Clone)]
pub enum GoToPage {
    GoToHomePage,
    GoToLogin,
    GoToDocsPage,
    GoToRegister,
    GoToAboutProject,
    GoToPrivacyAndSecurity
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