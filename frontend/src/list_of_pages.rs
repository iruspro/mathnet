/* 
In this file pages of the app are listed.
This file is also meant for proper routing and also managing url-s. */

use crate::app::DefinedMsg;
use crate::logics::special_parsing;
use crate::pages::logged_in_pages::exercise;
pub use crate::structs::chat_message::{ChatId};
pub use crate::structs::{user::{UserId},group_struct::{GroupId}, exercise::{ExerciseId},list_of_exercises::ListOfExercisesId};
pub use crate::logics::special_parsing::parse_from_str_to_u32;


#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    ItemUnsignedPage(UnsignedPage),
    ItemSignedPage(SignedPage),
    ItemOtherPage(OtherPage),
    ItemSharedPage(SharedPage),
}



pub fn page_name_to_string(page_name: Page) -> String {
    match page_name {
        // Unsigned pages
        Page::ItemUnsignedPage(UnsignedPage::Home) => "Home".to_string(),
        Page::ItemUnsignedPage(UnsignedPage::Login) => "Log in".to_string(),
        Page::ItemUnsignedPage(UnsignedPage::Register) => "Register".to_string(),
        //Signed pages
        Page::ItemSignedPage(SignedPage::GroupsList) => "Groups list".to_string(),
        Page::ItemSignedPage(SignedPage::UserProfile(_)) => "User profile".to_string(),
        Page::ItemSignedPage(SignedPage::Settings) => "Settings".to_string(),
        Page::ItemSignedPage(SignedPage::Notifications) => "Notifications".to_string(),
        Page::ItemSignedPage(SignedPage::ChatWithFriends) => "Chat with friends".to_string(),
        Page::ItemSignedPage(SignedPage::Chat(_)) => "Chat".to_string(),
        Page::ItemSignedPage(SignedPage::Friends) => "Friends".to_string(),
        Page::ItemSignedPage(SignedPage::Profile(_)) => "Profile".to_string(),
        Page::ItemSignedPage(SignedPage::Exercise(_,_)) => "Exercise".to_string(),
        Page::ItemSignedPage(SignedPage::ListOfExercises(_)) => "List_of_exercises".to_string(),
        Page::ItemSignedPage(SignedPage::Group(_)) => "Group".to_string(),
        Page::ItemSignedPage(SignedPage::UserSuggestsToDevelopers) => "Suggest to the developers".to_string(),
       
       //Other pages
        Page::ItemOtherPage(OtherPage::LogOut) => "Log out".to_string(),
        Page::ItemOtherPage(OtherPage::DeleteAccount) => "Delete account".to_string(),
        Page::ItemOtherPage(OtherPage::SuccessfullyChangedUserProfileData) => {
            "Successfully changed user profile data".to_string()
        }
        Page::ItemOtherPage(OtherPage::RetryChangingUserProfileData) => {
            "Retry changing user profile data".to_string()
        }
        Page::ItemOtherPage(OtherPage::NotFound) => {"Page not found".to_string()},
        //Shared pages
        Page::ItemSharedPage(SharedPage::Docs) => "Docs".to_string(),
        Page::ItemSharedPage(SharedPage::AboutProject) => "About this project".to_string(),
        Page::ItemSharedPage(SharedPage::PrivacyAndSecurity) => {
            "Privacy and security".to_string()
    }
}}


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
    Friends,
    Profile(UserId) ,
    Group(GroupId),
    Exercise(ListOfExercisesId,ExerciseId),
    ListOfExercises(ListOfExercisesId),
    UserSuggestsToDevelopers,
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

