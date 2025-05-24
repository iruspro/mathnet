use crate::list_of_pages::{Page, SharedPage, SignedPage, UnsignedPage};
use crate::logics::{login_logics, registration_logics};
pub use crate::messages::{
    DefinedMsg, UserLoginAttempt, UserRegister,
};
use crate::pages::{logged_in_pages, logged_out_pages, other_pages, shared_pages};
use crate::structs::{
    group_struct::*,
    user::{
        User, UserChangingProfileData, UserDemandsUserProfileDataChanges, UserLoginData,
        UserRegisterData, get_user_register_data,
    },
};
use crate::web_sys::console;
use crate::{
    list_of_pages::OtherPage, logics::registration_logics::check_registration_validity,
    messages::*, pages::logged_in_pages::chat, structs::chat_message::ChatId,
};
use sauron::dom::Program;
use sauron::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{Window, window, HashChangeEvent};
use sauron::{html::{attributes::*, events::*, *},Cmd,Application,Effects};
use crate::routing;

// (Main) model of this project
#[derive(Debug)]
pub struct App {
    pub current_page: Page, // routing for pages
    pub user_login_data: UserLoginData,
    pub user_register_data: UserRegisterData,
    pub user_data: User,
    pub user_changing_data_structure: UserChangingProfileData,
    pub current_conversation: Option<ChatId>,
    pub reload_current_page: bool,
    pub is_user_logged_in: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            current_page: Page::ItemUnsignedPage(UnsignedPage::Home),
            user_login_data: UserLoginData {
                user_name: String::new(),
                user_password: String::new(),
            },
            user_register_data: crate::structs::user::get_user_register_data(),
            user_data: crate::structs::user::new(),
            user_changing_data_structure: crate::structs::user::UserChangingProfileData::new(),
            current_conversation: None,
            reload_current_page: false,
            is_user_logged_in: false
        }
    }
    
}


impl Application for App {
    type MSG = DefinedMsg;
    fn init(&mut self) -> Cmd<DefinedMsg> {
    let current_hash = window()
        .and_then(|win| win.location().hash().ok())
        .unwrap_or_default();
    // Probably there exists more convenient way to handle this, but for some reason this code doesn't work in any other way.
    let new_effects = Effects::new(vec![DefinedMsg::ChangingHash(Page::parse(&current_hash.as_str()))],vec![()] );
    Cmd::from(new_effects)
}

    fn update(&mut self, message: DefinedMsg) -> Cmd<DefinedMsg> {
        match message.clone() {
            DefinedMsg::SetPage(page) => {
                routing::change_hash_while_navigating(page.clone());
                routing::routing_page_messages(message, self)},
            DefinedMsg::LoginAttempt(UserLoginAttempt::UpdateUserName(name)) => 
            {},
            DefinedMsg::LoginAttempt(UserLoginAttempt::UpdateUserPassword(passw)) => {
                self.user_login_data.user_password = passw
            }
            DefinedMsg::LoginAttempt(UserLoginAttempt::CheckLoginValidy) => {
                login_logics::check_login_attempt_validity(self)
            }

            DefinedMsg::Registration(UserRegister::UpdateUserRegisterName(name)) => {
                self.user_register_data.user_name = name
            }
            DefinedMsg::Registration(UserRegister::UpdateUserRegisterPassword(passw)) => {
                self.user_register_data.user_password = passw
            }
            DefinedMsg::Registration(UserRegister::UpdateUserRegisterPasswordAgain(passw_again)) => {
                self.user_register_data.user_password_again = passw_again
            }
            DefinedMsg::Registration(UserRegister::UpdateUserRegisterEmail(email)) => {
                self.user_register_data.user_email = email
            }
            DefinedMsg::Registration(UserRegister::RegistrationAttempt) => {
                registration_logics::check_registration_validity(self)
            }

            DefinedMsg::UserWantsToChangeProfileData(
                UserDemandsUserProfileDataChanges::ChangeUserName(new_name),
            ) => self.user_changing_data_structure.new_user_name = new_name,
            DefinedMsg::UserWantsToChangeProfileData(
                UserDemandsUserProfileDataChanges::ChangeUserEmail(new_email),
            ) => self.user_changing_data_structure.new_user_email = new_email,
            DefinedMsg::UserWantsToChangeProfileData(
                UserDemandsUserProfileDataChanges::ChangeUserPassword(new_password),
            ) => self.user_changing_data_structure.new_user_password = new_password,
            DefinedMsg::UserWantsToChangeProfileData(
                UserDemandsUserProfileDataChanges::ChangeUserPasswordConfirmation(new_password),
            ) => {
                self.user_changing_data_structure
                    .new_user_password_confirmation = new_password
            }
            DefinedMsg::UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges::DeleteAccount) => {
                self.current_page = Page::ItemOtherPage(OtherPage::DeleteAccount)
            }
            DefinedMsg::UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges::Retry) => {
                self.current_page = Page::ItemOtherPage(OtherPage::RetryChangingUserProfileData)
            }
            DefinedMsg::UserWantsToChangeProfileData(
                UserDemandsUserProfileDataChanges::ConfirmChanges,
            ) => {}
            DefinedMsg::SearchFriend(searched_person) => {
                unimplemented!()
            }
            DefinedMsg::NoAction => {
                unimplemented!()
            }
            DefinedMsg::UserWantsToChatWithSomePerson(user_id) => {
                unimplemented!()
            }
            DefinedMsg::UserWantsToChatWithSomePersonViaPersonalConversation(chat_id) => {
                self.current_conversation = Some(chat_id);
                self.current_page = Page::ItemSignedPage(SignedPage::Chat(chat_id));
            }
            DefinedMsg::SetConversationToNone => {
                self.current_conversation = None;
                self.current_page = Page::ItemSignedPage(SignedPage::ChatWithFriends)
            }
            DefinedMsg::NoOp => {}
            DefinedMsg::SendConversationMessage(conversation_message) => {}
            DefinedMsg::ChangingHash(page) => {routing::routing_page_messages(message, self)}
            _ => todo!("Why did update function receive DefinedMsg::SetPage message?")}
        Cmd::none()}


    fn view(&self) -> Node<DefinedMsg> {
        match self.current_page {

        // Unsigned pages
            Page::ItemUnsignedPage(UnsignedPage::Home) => logged_out_pages::home::view(),
            Page::ItemUnsignedPage(UnsignedPage::Login) => logged_out_pages::login::view(&self),
            Page::ItemUnsignedPage(UnsignedPage::Register) => logged_out_pages::register::view(),

        // Signed pages
            Page::ItemSignedPage(SignedPage::GroupsList) => {
                console::log_1(&"Hello 4!".into());
                logged_in_pages::groups::view(&self)
            }
            Page::ItemSignedPage(SignedPage::Settings) => logged_in_pages::settings::view(&self),

            Page::ItemSignedPage(SignedPage::UserProfile(user_id)) => {
                logged_in_pages::user_profile::view(&self)
            }
            Page::ItemSignedPage(SignedPage::ChatWithFriends) => {
                logged_in_pages::chat_with_friends::view(&self)
            }
            Page::ItemSignedPage(SignedPage::Notifications) => {
                logged_in_pages::notifications::view(&self)
            }
            Page::ItemSignedPage(SignedPage::Chat(chat_id)) => {
                console::log_1(&"Rust8!".into());
                logged_in_pages::chat::view(&self)
            }
            Page::ItemSignedPage(SignedPage::Friends) => {
                logged_in_pages::friends::view()
            }
            Page::ItemSignedPage(SignedPage::Profile(user_id)) => {
                logged_in_pages::profile::view()
            },

            // Shared pages
            Page::ItemSharedPage(SharedPage::AboutProject) => shared_pages::about_project::view(&self),
            Page::ItemSharedPage(SharedPage::Docs) => shared_pages::docs::view(&self),
            Page::ItemSharedPage(SharedPage::PrivacyAndSecurity) => shared_pages::privacy_and_security::view(&self),

            //Other pages
            Page::ItemOtherPage(OtherPage::SuccessfullyChangedUserProfileData) => {
                other_pages::successfully_changed_user_profile_data::view()
            }
            Page::ItemOtherPage(OtherPage::RetryChangingUserProfileData) => {
                other_pages::retry_changing_user_profile_data::view()
            }
            Page::ItemOtherPage(OtherPage::DeleteAccount) => {
                other_pages::delete_account::view()
            }
            Page::ItemOtherPage(OtherPage::LogOut) => other_pages::log_out::view(),
        }
    }
}

//Testing

/*
#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}
*/

#[wasm_bindgen(start)]
pub fn start() {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
    use web_sys::{window, HashChangeEvent};
    use sauron::dom::Program;

    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    let app = App::new();
    let mut program = Program::mount_to_body(app);

    // Setup hashchange listener
    let closure = Closure::wrap(Box::new(move |_: HashChangeEvent| {
        if let Some(hash) = window().and_then(|win| win.location().hash().ok()) {
            program.dispatch(DefinedMsg::ChangingHash(Page::parse(&hash)));
        }
    }) as Box<dyn FnMut(_)>);

    window()
        .unwrap()
        .add_event_listener_with_callback("hashchange", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget(); // Keeps the closure alive
}
