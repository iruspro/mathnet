/* This page is for proper routing and also manages url-s. */

pub use crate::structs::chat_message::ChatId;

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    ItemUnsignedPage(UnsignedPage),
    ItemSignedPage(SignedPage),
    ItemOtherPage(OtherPage),
}

impl Page {
    pub fn parse(hash: &str) -> Self {
        let hash = hash.trim_start_matches('#').trim_start_matches('/');
        let segments: Vec<&str> = hash.split('/').collect();

        match segments.as_slice() {
            // logout pages
            ["home"] => Page::ItemUnsignedPage(UnsignedPage::Home),
            ["login"] => Page::ItemUnsignedPage(UnsignedPage::Login),
            ["privacy_and_security"] => Page::ItemUnsignedPage(UnsignedPage::PrivacyAndSecurity),
            ["register"] => Page::ItemUnsignedPage(UnsignedPage::Register),

            // login pages
            ["chat_with_friends", user_id] =>
            ["conversation", chat_id] =>
            ["exercise", exercise_id] =>
            ["friends"] =>
            ["groups"] =>
            ["group", group_id] =>
            ["list_of_exercises", list_of_exercises_id] => 
            ["notifications", user_id] =>
            ["settings"] =>
            ["profile", user_id] =>
            ["user_profile", user_id] =>

            // other pages
            ["delete_account"] =>
            ["log_out"] => 
            ["retry_changing_user_profile_data", user_id] => 
            ["successfully_changed_user_profile_data", user_id] => 


            // shared pages
            ["login"] => Page::ItemUnsignedPage(UnsignedPage::Login),
            ["docs"] => Page::ItemUnsignedPage(UnsignedPage::Docs),
            ["about_project"] => Page::ItemUnsignedPage(UnsignedPage::AboutProject),






            ["profile"] => Route::Profile,
            ["post", id] => id.parse().ok().map(Route::Post).unwrap_or(Route::NotFound),
            _ => Route::NotFound,
        }
    }

    pub fn to_hash(&self) -> String {
        match self {
            Route::Home => "#/home".into(),
            Route::Profile => "#/profile".into(),
            Route::Post(id) => format!("#/post/{}", id),
            Route::NotFound => "#/404".into(),
        }
    }
}























    pub fn page_name_to_string(page_name: Page) -> String {
        match page_name {
            Page::ItemUnsignedPage(UnsignedPage::Home) => "Home".to_string(),
            Page::ItemUnsignedPage(UnsignedPage::Login) => "Log in".to_string(),
            Page::ItemUnsignedPage(UnsignedPage::Register) => "Register".to_string(),
            Page::ItemUnsignedPage(UnsignedPage::Docs) => "Docs".to_string(),
            Page::ItemUnsignedPage(UnsignedPage::AboutProject) => "About this project".to_string(),
            Page::ItemUnsignedPage(UnsignedPage::PrivacyAndSecurity) => {
                "Privacy and security".to_string()
            }
            Page::ItemSignedPage(SignedPage::GroupsList) => "Groups list".to_string(),
            Page::ItemSignedPage(SignedPage::Docs) => "Docs".to_string(),
            Page::ItemSignedPage(SignedPage::AboutProject) => "About this project".to_string(),
            Page::ItemSignedPage(SignedPage::PrivacyAndSecurity) => {
                "Privacy and security".to_string()
            }
            Page::ItemSignedPage(SignedPage::UserProfile) => "User profile".to_string(),
            Page::ItemSignedPage(SignedPage::Settings) => "Settings".to_string(),
            Page::ItemSignedPage(SignedPage::Notifications) => "Notifications".to_string(),
            Page::ItemSignedPage(SignedPage::ChatWithFriends) => "Chat with friends".to_string(),
            Page::ItemSignedPage(SignedPage::Conversation) => "Converation".to_string(),
            Page::ItemSignedPage(SignedPage::LogOut) => "Log out".to_string(),
            Page::ItemOtherPage(OtherPage::LogOut) => "Log out".to_string(),

            Page::ItemOtherPage(OtherPage::DeleteAccount) => "Delete account".to_string(),
            Page::ItemOtherPage(OtherPage::SuccessfullyChangedUserProfileData) => {
                "Successfully changed user profile data".to_string()
            }
            Page::ItemOtherPage(OtherPage::RetryChangingUserProfileData) => {
                "Retry changing user profile data".to_string()
            }
        }
    }

#[derive(Debug, Clone, PartialEq)]
pub enum UnsignedPage {
    Home,
    Login,
    Register,
    Docs,
    AboutProject,
    PrivacyAndSecurity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SignedPage {
    GroupsList,
    Docs,
    AboutProject,
    PrivacyAndSecurity,
    UserProfile,
    Settings,
    Notifications,
    ChatWithFriends,
    Conversation,
    LogOut,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OtherPage {
    SuccessfullyChangedUserProfileData,
    RetryChangingUserProfileData,
    DeleteAccount,
    LogOut,
}
