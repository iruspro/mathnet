use sauron::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window;
use crate::{list_of_pages::OtherPage, logics::registration_logics::check_registration_validity, messages::*};
pub use crate::messages::{Msg, UserLoginAttempt,UserRegister};
use crate::list_of_pages::{Page,UnsignedPage,SignedPage};
use crate::pages::pages_templates::{logged_in_page_template::logged_in_page_template, logged_out_page_template::logged_out_page_template};
use sauron::dom::Program;
use crate::structs::{user::{UserLoginData, UserRegisterData,get_user_register_data,User,UserDemandsUserProfileDataChanges,UserChangingProfileData},group_struct::*};
use crate::logics::{login_logics,registration_logics};
use crate::web_sys::console;
use crate::structs::chat_message::ChatId;

#[derive(Debug)]
pub struct App {
    pub current_page: Page,
    pub user_login_data : UserLoginData,
    pub user_register_data : UserRegisterData,
    pub user_data : User,
    pub user_changing_data_structure : UserChangingProfileData,
    pub current_conversation : Option<ChatId>,
    pub reload_current_page : bool,
}

impl App {
    pub fn new() -> Self {
        App {
            current_page : Page::PageSortUnsigned(UnsignedPage::Home),
            user_login_data : UserLoginData{user_name : String::new(), user_password : String::new()},
            user_register_data : crate::structs::user::get_user_register_data(),
            user_data : crate::structs::user::new(),
            user_changing_data_structure : crate::structs::user::UserChangingProfileData::new(),
            current_conversation : None,
            reload_current_page : false,
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
            
            // TODO: Update pages according to changed page variants in 'list_of_pages.rs'
            Msg::SetPage(page) => self.current_page = page,

            Msg::LoginAttempt(UserLoginAttempt::UpdateUserName(name)) => self.user_login_data.user_name = name,
            Msg::LoginAttempt(UserLoginAttempt::UpdateUserPassword(passw)) => self.user_login_data.user_password = passw,
            Msg::LoginAttempt(UserLoginAttempt::CheckLoginValidy) => login_logics::check_login_attempt_validity(self),
            
            Msg::Registration(UserRegister::UpdateUserRegisterName(name)) => self.user_register_data.user_name = name, 
            Msg::Registration(UserRegister::UpdateUserRegisterPassword(passw)) => self.user_register_data.user_password = passw,
            Msg::Registration(UserRegister::UpdateUserRegisterPasswordAgain(passw_again)) =>self.user_register_data.user_password_again = passw_again,
            Msg::Registration(UserRegister::UpdateUserRegisterEmail(email)) => self.user_register_data.user_email = email,
            Msg::Registration(UserRegister::RegistrationAttempt) => registration_logics::check_registration_validity(self),
        
            Msg::UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges::ChangeUserName(new_name)) => self.user_changing_data_structure.new_user_name = new_name,
            Msg::UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges::ChangeUserEmail(new_email)) => self.user_changing_data_structure.new_user_email = new_email,
            Msg::UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges::ChangeUserPassword(new_password)) => self.user_changing_data_structure.new_user_password = new_password,
            Msg::UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges::ChangeUserPasswordConfirmation(new_password)) => self.user_changing_data_structure.new_user_password_confirmation = new_password,
            Msg::UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges::DeleteAccount) => {self.current_page = Page::PageSortOther(OtherPage::DeleteAccount)},
            Msg::UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges::Retry) => self.current_page = Page::PageSortOther(OtherPage::RetryChangingUserProfileData),
            Msg::UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges::ConfirmChanges) => {},
            Msg::SearchFriend(searched_person) => {unimplemented!()},
            Msg::NoAction => {unimplemented!()},
            Msg::UserWantsToChatWithSomePerson(user_id) => {unimplemented!()},
            Msg::UserWantsToChatWithSomePersonViaPersonalConversation(chat_id) => {self.current_conversation = Some(chat_id); self.current_page = Page::PageSortSigned(SignedPage::Conversation)},
            Msg::SetConversationToNone => {self.current_conversation = None; self.current_page = Page::PageSortSigned(SignedPage::ChatWithFriends)},
            Msg::NoOp =>{},
            Msg::SendConversationMessage(conversation_message) => {},
        
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Self::MSG> {
        // TODO: fix variants so that they will match simplified variants of page names
        // The change from 'view' methods to just triggering page_templates and insertion 
        // of pages' content is also needed.
        match &self.current_page {
            Page::PageSortUnsigned(page) => logged_out_page_template(&Page::PageSortUnsigned(page.clone())),
            Page::PageSortSigned(page) => logged_in_page_template(&Page::PageSortSigned(page.clone())),
            Page::PageSortOther(page) => todo!("implement matching for other pages"),
        }
    }

    }
    //Testing 

    #[wasm_bindgen(start)]
    pub fn start() {
        console_log::init_with_level(log::Level::Trace).unwrap();
        console_error_panic_hook::set_once();
        Program::mount_to_body(App::new());
    }
