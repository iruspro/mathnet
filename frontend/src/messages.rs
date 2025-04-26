use crate::user::UserLoginData;

#[derive(Debug,Clone)]
pub enum Msg {
    SetPage(GoToPage),
    LoginAttempt(UserLoginAttempt),
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