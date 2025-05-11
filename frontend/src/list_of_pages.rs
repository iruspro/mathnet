pub use crate::structs::chat_message::ChatId;

#[derive(Debug,Clone,PartialEq)]
pub enum Page {
    ItemUnsignedPage(UnsignedPage),
    ItemSignedPage(SignedPage),
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
LogOut,
Notifications,
ChatWithFriends,
SuccessfullyChangedUserProfileData,
RetryChangingUserProfileData,
DeleteAccount,
BothSidebars,
Conversation(ChatId),
}
