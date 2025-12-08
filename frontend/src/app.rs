/*
Main frontend file.
*/

use sauron::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window;
use crate::pages::pages_templates_and_routing::display_content_function::display_content;
use crate::{list_of_pages::OtherPage, logics::registration_logics::check_registration_validity, messages::*};
pub use crate::messages::{Msg, UserLoginAttempt,UserRegister};
use crate::list_of_pages::{Page,UnsignedPage,SignedPage};
use crate::pages::pages_templates_and_routing::{signed_in_page_template::signed_in_page_template, 
                                                signed_out_page_template::signed_out_page_template,
                                                other_page_router::other_page_router,
                                                shared_page_router::shared_page_router};
use sauron::dom::Program;
use crate::structs::{user::{UserLoginData, UserRegisterData,get_user_register_data,User,UserDemandsUserProfileDataChanges,UserChangingProfileData},group_struct::*};
use crate::logics::{login_logics,registration_logics};
use crate::web_sys::console;
use crate::frontend_features::delete_account::delete_account;
use crate::frontend_features::friends::find_people::FriendSearchQuery;
use crate::frontend_features::friends::find_people::PersonSearchQuery;


// Application state
#[derive(Debug)]
pub struct App {
    pub current_page: Page,
    pub is_user_signed_in: bool,
    pub user_sign_in_data : UserLoginData,
    pub user_register_data : UserRegisterData,
    pub user_data : User,
    pub user_changing_data_structure : UserChangingProfileData,
    pub current_conversation : Option<u128>,
    pub reload_current_page : bool,
    pub person_search_query : String,
    pub person_search_results: Vec<PersonSearchQuery>,
    pub friend_search_query: String,
    pub friend_search_results: Vec<FriendSearchQuery>
}

impl App {
    pub fn new() -> Self {
        App {
            current_page : Page::PageSortUnsigned(UnsignedPage::Home),
            is_user_signed_in: false,
            user_sign_in_data : UserLoginData{user_name : String::new(), user_password : String::new()},
            user_register_data : crate::structs::user::get_user_register_data(),
            user_data : crate::structs::user::new(),
            user_changing_data_structure : crate::structs::user::UserChangingProfileData::new(),
            current_conversation : None,
            reload_current_page : false,
            person_search_query: String::new(),
            person_search_results: Vec::new(),
            friend_search_query: String::new(),
            friend_search_results: Vec::new()
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

    // Update application's state based on a message it receives.
    /*
    TODO: include missing messages and make subsections based on 
    their functionality.
    */
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
            Msg::SearchFriend => {},
            Msg::NoAction => {unimplemented!()},
            Msg::UserWantsToChatWithSomePerson(user_id) => {unimplemented!()},
            Msg::UserWantsToChatWithSomePersonViaPersonalConversation(conversation_id) => {self.current_conversation = Some(conversation_id); self.current_page = Page::PageSortSigned(SignedPage::Conversation)},
            Msg::SetConversationToNone => {self.current_conversation = None; self.current_page = Page::PageSortSigned(SignedPage::ConversationWithFriends)},
            Msg::NoOp =>{},
            Msg::SendConversationMessage(conversation_message) => {},
            Msg::DeleteAccount => {
                self.current_page = Page::PageSortUnsigned(UnsignedPage::Home);
                delete_account();
            }
            Msg::UpdatePersonSearch(s) => {self.person_search_query = s},
            Msg::UpdateFriendSearch(s) => {self.friend_search_query = s}
            Msg::AcceptFriendRequest => {},
            Msg::RejectFriendRequest => {},
            Msg::CancelFriendRequest => {},        
        }
        Cmd::none()
    }

    // Show content on browser.
    fn view(&self) -> Node<Self::MSG> {
        // TODO: fix variants so that they will match simplified variants of page names
        // The change from 'view' methods to just triggering page_templates and insertion 
        // of pages' content is also needed.
        match self.current_page {
            Page::PageSortUnsigned(_) => signed_out_page_template(&self.current_page),
            Page::PageSortSigned(_)   => signed_in_page_template(&self.current_page),
            Page::PageSortShared(_)   => shared_page_router(self, &self.current_page),
            Page::PageSortOther(_)    => other_page_router(self, &self.current_page), // Will be fixed when implementing other_page_router

        }
    }

    }
   
    // A function that actually starts frontend. 
    #[wasm_bindgen(start)]
    pub fn start() {
        console_log::init_with_level(log::Level::Trace).unwrap();
        console_error_panic_hook::set_once();
        Program::mount_to_body(App::new());
    }
