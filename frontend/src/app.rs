use sauron::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window;
use crate::logics::registration_logics::check_registration_validity;
use crate::messages::{GoToPage, Msg, UserLoginAttempt,UserRegister};
use crate::list_of_pages::{self, Page};
use crate::pages::{logged_out_pages::{about_project,docs,home,privacy_and_security,login,register},logged_in_pages::{chat,exercise,list_of_exercises,settings,user_profile}};
use sauron::dom::Program;
use crate::user::{UserLoginData, UserRegisterData,get_user_register_data};
use crate::logics::{login_logics,registration_logics};
use crate::user;

pub struct App {
    pub current_page: Page,
    pub user_login_data : UserLoginData,
    pub user_register_data : UserRegisterData,
}

impl App {
    pub fn new() -> Self {
        App {
            current_page : Page::Home,
            user_login_data : UserLoginData{user_name : String::new(), user_password : String::new()},
            user_register_data : user::get_user_register_data()
        }
    }
    //fn parse_location() -> Page {
    //    let path = window().unwrap().location().pathname().unwrap_or_default();
    //    match path.as_str() {
    //        "/" => Page::Home,
    //        "/docs" => Page::Docs,
    //        "/chat" => Page::Chat,
    //        "/exercise" => Page::Exercise,
    //        "/list_of_exercises" => Page::ListOfExercise,
    //        "/login" => Page::Login,
    //        "/register" => Page::Register,
    //        "/settings" => Page::Settings,
    //        "/user_profile" => Page::UserProfile,
    //        _ => Page::Home,
    //    }
    //}
}//

impl Application for App {
    type MSG = Msg;
    fn update(&mut self, msg: Self::MSG) -> Cmd<Msg> {
        match msg {
            Msg::SetPage(GoToPage::GoToHomePage) => self.current_page = Page::Home,
            Msg::SetPage(GoToPage::GoToDocsPage) => self.current_page = Page::Docs,
            Msg::SetPage(GoToPage::GoToLogin )=> self.current_page = Page::Login,
            Msg::SetPage(GoToPage::GoToRegister) => self.current_page = Page::Register,
            Msg::SetPage(GoToPage::GoToAboutProject) => self.current_page = Page::AboutProject,
            Msg::SetPage(GoToPage::GoToPrivacyAndSecurity) => self.current_page = Page::PrivacyAndSecurity,
        
            Msg::LoginAttempt(UserLoginAttempt::UpdateUserName(name)) => self.user_login_data.user_name = name,
            Msg::LoginAttempt(UserLoginAttempt::UpdateUserPassword(passw)) => self.user_login_data.user_password = passw,
            Msg::LoginAttempt(UserLoginAttempt::CheckLoginValidy) => login_logics::check_login_attempt_validity(self),
            
            Msg::Registration(UserRegister::UpdateUserRegisterName(name)) => self.user_register_data.user_name = name, 
            Msg::Registration(UserRegister::UpdateUserRegisterPassword(passw)) => self.user_register_data.user_password = passw,
            Msg::Registration(UserRegister::UpdateUserRegisterPasswordAgain(passw_again)) =>self.user_register_data.user_password_again = passw_again,
            Msg::Registration(UserRegister::UpdateUserRegisterEmail(email)) => self.user_register_data.user_email = email,
            Msg::Registration(UserRegister::RegistrationAttempt) => registration_logics::check_registration_validity(self),
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Self::MSG> {
        match self.current_page {
            Page::Home => home::view(),
            Page::Chat => chat::view(),
            Page::Docs => docs::view(),
            Page::Exercise => exercise::view(),
            Page::ListOfExercise => list_of_exercises::view(),
            Page::Login => login::view(&self),
            Page::Register => register::view(),
            Page::Settings => settings::view(),
            Page::UserProfile => user_profile::view(),
            Page::AboutProject => about_project::view(),
            Page::PrivacyAndSecurity => privacy_and_security::view(),
        }
    }

    }

    #[wasm_bindgen(start)]
    pub fn start() {
        console_log::init_with_level(log::Level::Trace).unwrap();
        console_error_panic_hook::set_once();
        Program::mount_to_body(App::new());
    }
