pub use crate::structs::chat_message::ChatId;

#[derive(Debug,Clone,PartialEq)]
pub enum Page {
    ItemUnsignedPage(UnsignedPage),
    ItemSignedPage(SignedPage),
    ItemOtherPage(OtherPage),
}

impl Page {
    pub fn page_name_to_string(page_name : Page) -> String{
        match page_name{

            Page::ItemUnsignedPage(UnsignedPage::Home) => "Home".to_string(),
            Page::ItemUnsignedPage(UnsignedPage::Login) => "Log in".to_string(),
            Page::ItemUnsignedPage(UnsignedPage::Register) => "Register".to_string(),
            Page::ItemUnsignedPage(UnsignedPage::Docs) => "Docs".to_string(),
            Page::ItemUnsignedPage(UnsignedPage::AboutProject) => "About this project".to_string(),
            Page::ItemUnsignedPage(UnsignedPage::PrivacyAndSecurity) => "Privacy and security".to_string(),
            Page::ItemSignedPage(SignedPage::GroupsList) => "Groups list".to_string(),
            Page::ItemSignedPage(SignedPage::Docs) => "Docs".to_string(),
            Page::ItemSignedPage(SignedPage::AboutProject) => "About this project".to_string(),
            Page::ItemSignedPage(SignedPage::PrivacyAndSecurity) => "Privacy and security".to_string(),
            Page::ItemSignedPage(SignedPage::UserProfile) => "User profile".to_string(),
            Page::ItemSignedPage(SignedPage::Settings) => "Settings".to_string(),
            Page::ItemSignedPage(SignedPage::Notifications) => "Notifications".to_string(),
            Page::ItemSignedPage(SignedPage::ChatWithFriends) => "Chat with friends".to_string(),
            Page::ItemSignedPage(SignedPage::Conversation) => "Converation".to_string(),
            Page::ItemSignedPage(SignedPage::LogOut) => "Log out".to_string(),
            Page::ItemOtherPage(OtherPage::LogOut) => "Log out".to_string(),
         
            Page::ItemOtherPage(OtherPage::DeleteAccount) => "Delete account".to_string(),
            Page::ItemOtherPage(OtherPage::SuccessfullyChangedUserProfileData) => "Successfully changed user profile data".to_string(),
            Page::ItemOtherPage(OtherPage::RetryChangingUserProfileData) => "Retry changing user profile data".to_string(),

        }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum UnsignedPage {
Home,
Login,
Register,
Docs,
AboutProject,
PrivacyAndSecurity,
}

#[derive(Debug,Clone,PartialEq)]
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
LogOut
}

#[derive(Debug,Clone,PartialEq)]
pub enum OtherPage{
SuccessfullyChangedUserProfileData,
RetryChangingUserProfileData,
DeleteAccount,
LogOut,
}