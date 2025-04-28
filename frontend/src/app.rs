use sauron::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window;
use crate::logics::registration_logics::check_registration_validity;
pub use crate::messages::{Msg, UserLoginAttempt,UserRegister,SwitchToPageSigned,SwitchToPageUnsigned,GoToPage::{GoToPageSigned, GoToPageUnsigned}};
use crate::list_of_pages::{self, Page,UnsignedPage,SignedPage};
use crate::pages::{logged_out_pages,logged_in_pages};
use sauron::dom::Program;
use crate::user::{UserLoginData, UserRegisterData,get_user_register_data};
use crate::logics::{login_logics,registration_logics};
use crate::user;
use crate::messages::GoToPage;

pub struct App {
    pub current_page: Page,
    pub user_login_data : UserLoginData,
    pub user_register_data : UserRegisterData,
}

impl App {
    pub fn new() -> Self {
        App {
            current_page : Page::ItemUnsignedPage(UnsignedPage::Home),
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
            Msg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToHomePage)) => self.current_page = Page::ItemUnsignedPage(UnsignedPage::Home),
            Msg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToDocsPage)) => self.current_page = Page::ItemUnsignedPage(UnsignedPage::Docs),
            Msg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToLoginPage) )=> self.current_page = Page::ItemUnsignedPage(UnsignedPage::Login),
            Msg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToRegister)) => self.current_page = Page::ItemUnsignedPage(UnsignedPage::Register),
            Msg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToAboutProject)) => self.current_page = Page::ItemUnsignedPage(UnsignedPage::AboutProject),
            Msg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToPrivacyAndSecurity)) => self.current_page = Page::ItemUnsignedPage(UnsignedPage::PrivacyAndSecurity),
            
            Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroupsList)) => self.current_page = Page::ItemSignedPage(SignedPage::GroupsList),
            Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToAboutProject)) => self.current_page = Page::ItemSignedPage(SignedPage::AboutProject),
            Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToPrivacyAndSecurity)) => self.current_page = Page::ItemSignedPage(SignedPage::PrivacyAndSecurity),
            Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile)) => self.current_page = Page::ItemSignedPage(SignedPage::UserProfile),
            Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToSettings)) => self.current_page = Page::ItemSignedPage(SignedPage::Settings),
            Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToDocsPage)) => self.current_page = Page::ItemSignedPage(SignedPage::Docs),
            Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToLogOut)) => self.current_page = Page::ItemSignedPage(SignedPage::LogOut),        



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
            Page::ItemUnsignedPage(UnsignedPage::Home) => logged_out_pages::home::view(),
            Page::ItemUnsignedPage(UnsignedPage::Docs) => logged_out_pages::docs::view(),
            Page::ItemUnsignedPage(UnsignedPage::AboutProject) => logged_out_pages::about_project::view(),
            Page::ItemUnsignedPage(UnsignedPage::Login) => logged_out_pages::login::view(&self),
            Page::ItemUnsignedPage(UnsignedPage::PrivacyAndSecurity) => logged_out_pages::privacy_and_security::view(),
            Page::ItemUnsignedPage(UnsignedPage::Register) => logged_out_pages::register::view(),

            Page::ItemSignedPage(SignedPage::GroupsList) => logged_in_pages::groups::view(),
            Page::ItemSignedPage(SignedPage::Settings) => logged_in_pages::settings::view(),
            Page::ItemSignedPage(SignedPage::UserProfile) => logged_in_pages::user_profile::view(),
            Page::ItemSignedPage(SignedPage::PrivacyAndSecurity) => logged_in_pages::groups::view(),
            Page::ItemSignedPage(SignedPage::GroupsList) => logged_in_pages::groups::view(),
            Page::ItemSignedPage(SignedPage::AboutProject) => logged_in_pages::groups::view(),
            Page::ItemSignedPage(SignedPage::LogOut) => logged_in_pages::log_out::view(), 
            Page::ItemSignedPage(SignedPage::Docs) => logged_in_pages::groups::view(),    
        }
    }

    }

    #[wasm_bindgen(start)]
    pub fn start() {
        console_log::init_with_level(log::Level::Trace).unwrap();
        console_error_panic_hook::set_once();
        Program::mount_to_body(App::new());
    }
