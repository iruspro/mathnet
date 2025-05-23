/* This page is for proper routing and also manages url-s. */

use crate::logics::special_parsing;
pub use crate::structs::chat_message::{ChatId};
pub use crate::structs::{user::{UserId},group_struct::{GroupId}, exercise::{ExerciseId}};
pub use crate::logics::special_parsing::parse_from_str_to_u32;


#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    ItemUnsignedPage(UnsignedPage),
    ItemSignedPage(SignedPage),
    ItemOtherPage(OtherPage),
    ItemSharedPage(SharedPage),
}

impl Page {
    pub fn parse(hash: &str) -> Self {
        let hash = hash.trim_start_matches('#').trim_start_matches('/');
        let segments: Vec<&str> = hash.split('/').collect();

        match segments.as_slice() {
            // logout pages
            ["home"] => Page::ItemUnsignedPage(UnsignedPage::Home),
            ["login"] => Page::ItemUnsignedPage(UnsignedPage::Login),
            ["register"] => Page::ItemUnsignedPage(UnsignedPage::Register),

            // login pages
            ["chat_with_friends"] => Page::ItemSignedPage(SignedPage::ChatWithFriends),
            ["chat", chat_id] => Page::ItemSignedPage(SignedPage::Chat(ChatId::ChatIdNumber(parse_from_str_to_u32(chat_id)))),
            ["exercise", exercise_id] => Page::ItemSignedPage(SignedPage::Exercise(ExerciseId::ExerciseIdNumber(parse_from_str_to_u32(exercise_id)))),
            ["groups"] => Page::ItemSignedPage(SignedPage::GroupsList),
            ["group", group_id] => Page::ItemSignedPage(SignedPage::Group(GroupId::GroupId(parse_from_str_to_u32(group_id)))),
            ["list_of_exercises", list_of_exercises_id] => ///////////////////////////////////////// Remained here!!!!!!!!!!!!!!!!!
            ["notifications", user_id] =>Page::ItemSignedPage(SignedPage::Notifications),
            ["settings"] => Page::ItemSignedPage(SignedPage::Settings),
            ["user_profile", user_id] =>Page::ItemSignedPage(SignedPage::UserProfile(user_id)),
            ["friends"] => Page::ItemSignedPage(SignedPage::Friends),
            ["profile", user_id] => Page::ItemSignedPage(SignedPage::Profile(user_id)),

            // other pages
            ["delete_account"] => Page::ItemOtherPage(OtherPage::DeleteAccount),
            ["logout"] => Page::ItemOtherPage(OtherPage::LogOut), 
            ["retry_changing_user_profile_data"] => Page::ItemOtherPage(OtherPage::RetryChangingUserProfileData),
            ["successfully_changed_user_profile_data"] => Page::ItemOtherPage(OtherPage::SuccessfullyChangedUserProfileData),


            // shared pages
            ["privacy_and_security"] => Page::ItemSharedPage(SharedPage::PrivacyAndSecurity),
            ["docs"] => Page::ItemSharedPage(SharedPage::Docs),
            ["about_project"] => Page::ItemSharedPage(SharedPage::AboutProject),

        }
    }

    pub fn to_hash(&self) -> String {
        match self {
            //Unsigned pages
            Page::ItemUnsignedPage(UnsignedPage::Home) => "#/home".into(),
            Page::ItemUnsignedPage(UnsignedPage::Login) => "#/login".into(),
            Page::ItemUnsignedPage(UnsignedPage::Register) => "#/register".into(),

            //Signed pages
            Page::ItemSignedPage(SignedPage::GroupsList) => "#/groups_list".into(),
            Page::ItemSignedPage(SignedPage::UserProfile(user_id)) => format!("#/user_profile/{:?}", user_id.clone())
            Page::ItemSignedPage(SignedPage::Settings) => "#/settings".into(),
            Page::ItemSignedPage(SignedPage::Notifications) => "#/notifications".into(),
            Page::ItemSignedPage(SignedPage::ChatWithFriends) => "#/chat_with_friends".into(),
            Page::ItemSignedPage(SignedPage::Chat(chat_id)) => format!("#/chat/{:?}", chat_id),
            Page::ItemSignedPage(SignedPage::Friends) => "#/friends".into(),
            Page::ItemSignedPage(SignedPage::Profile(user_id)) => format!("#/profile/{:?}", user_id),
           
           //Other pages

            Page::ItemOtherPage(OtherPage::LogOut) => "#/logout".into(),

            Page::ItemOtherPage(OtherPage::DeleteAccount) => "#/delete_account".into(),
            Page::ItemOtherPage(OtherPage::SuccessfullyChangedUserProfileData) => {
                "#/successfully_changed_user_profile_data".into()
            }
            Page::ItemOtherPage(OtherPage::RetryChangingUserProfileData) => {
                "#/retry_changing_user_profile_data".into()
            }
            Page::ItemOtherPage(OtherPage::NotFound) => {
                "#/404".into()
            }

            //Shared pages
            Page::ItemSharedPage(SharedPage::Docs) => "#/docs".into(),
            Page::ItemSharedPage(SharedPage::AboutProject) => "#/about_this_project".into(),
            Page::ItemSharedPage(SharedPage::PrivacyAndSecurity) => {
                "privacy_and_security".into()
        }
        }
    }
}






















/*
Currently meaningless piece of code

    pub fn page_name_to_string(page_name: Page) -> String {
        match page_name {
            // Unsigned pages
            Page::ItemUnsignedPage(UnsignedPage::Home) => "Home".to_string(),
            Page::ItemUnsignedPage(UnsignedPage::Login) => "Log in".to_string(),
            Page::ItemUnsignedPage(UnsignedPage::Register) => "Register".to_string(),

            //Signed pages
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
            Page::ItemSignedPage(SignedPage::Chat) => "Chat".to_string(),
            Page::ItemSignedPage(SignedPage::LogOut) => "Log out".to_string(),
            Page::ItemSignedPage(SignedPage::Friends) => "Friends".to_string(),
            Page::ItemSignedPage(SignedPage::Profile) => "Profile".to_string(),
           
           //Other pages

            Page::ItemOtherPage(OtherPage::LogOut) => "Log out".to_string(),

            Page::ItemOtherPage(OtherPage::DeleteAccount) => "Delete account".to_string(),
            Page::ItemOtherPage(OtherPage::SuccessfullyChangedUserProfileData) => {
                "Successfully changed user profile data".to_string()
            }
            Page::ItemOtherPage(OtherPage::RetryChangingUserProfileData) => {
                "Retry changing user profile data".to_string()
            }

            //Shared pages
            Page::ItemSharedPage(SharedPage::Docs) => "Docs".to_string(),
            Page::ItemSharedPage(SharedPage::AboutProject) => "About this project".to_string(),
            Page::ItemSharedPage(SharedPage::PrivacyAndSecurity) => {
                "Privacy and security".to_string()
        }
    }}
*/
#[derive(Debug, Clone, PartialEq)]
pub enum UnsignedPage {
    /* Pages that are visible to loged in users and 
    persons when they are not signed in or even don't have an account.*/
    Home,
    Login,
    Register,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SignedPage {
    /* Pages when a user is signed in */
    GroupsList,
    UserProfile(UserId),
    Settings,
    Notifications,
    ChatWithFriends,
    Chat(ChatId),
    LogOut,
    Friends,
    Profile(UserId) ,
    Group(GroupId),
    Exercise(ExerciseId),
    /* User profile is for managing user's personal data, while profile is for
    sharing personal thoughts and presenting viewer to general public */
}

#[derive(Debug, Clone, PartialEq)]
pub enum OtherPage {
    /* Auxiliary pages */
    SuccessfullyChangedUserProfileData,
    RetryChangingUserProfileData,
    DeleteAccount,
    LogOut,
    NotFound,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SharedPage{
    /* Pages that are common to both logged in and logged out state of app */
    AboutProject,
    Docs,
    PrivacyAndSecurity,


}