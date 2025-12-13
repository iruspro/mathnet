/* 
List of available pages on MathNet.
*/




#[derive(Debug,Clone,PartialEq)]
pub enum Page {
    PageSortUnsigned(UnsignedPage),
    PageSortSigned(SignedPage),
    PageSortOther(OtherPage),
    PageSortShared(SharedPage)
}

/*
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

*/

// Some kind of info pages for potential new users.
#[derive(Debug,Clone,PartialEq)]
pub enum UnsignedPage {
Home,
SignIn,
Register
}

// Accessible only when user is logged in.
#[derive(Debug,Clone,PartialEq)]
pub enum SignedPage {
    BaseOfExercises,
    BaseOfLearningResources,
    ChatWithFriends,
    Conversation,
    ConversationList,
    Exercise,
    Group,
    GroupList,
    LearningResources,
    MathNetNotifications,
    Settings,
    UserProfileManagement,
    UserProfileWall,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SharedPage{
    AboutProject,
    Docs,
    PrivacyAndSecurity,
}

// Pages that provide system messages or other important tasks not strictly related 
// to the main way of use.
#[derive(Debug,Clone,PartialEq)]
pub enum OtherPage{
DeleteAccount,
SignOut,
}

// This and the next array are for pattern matching. 
// Should be the same as SignedPage and UnsignedPage enums.
pub const list_of_signed_pages: [Page; 12] = [
    Page::PageSortSigned(SignedPage::BaseOfExercises),
    Page::PageSortSigned(SignedPage::BaseOfLearningResources),
    Page::PageSortSigned(SignedPage::ChatWithFriends),
    Page::PageSortSigned(SignedPage::Conversation),
    Page::PageSortSigned(SignedPage::Exercise),
    Page::PageSortSigned(SignedPage::Group),
    Page::PageSortSigned(SignedPage::GroupList),
    Page::PageSortSigned(SignedPage::LearningResources),
    Page::PageSortSigned(SignedPage::MathNetNotifications),
    Page::PageSortSigned(SignedPage::Settings),
    Page::PageSortSigned(SignedPage::UserProfileManagement),
    Page::PageSortSigned(SignedPage::UserProfileWall),
];

pub const list_of_unsigned_pages: [Page;3] = [
    Page::PageSortUnsigned(UnsignedPage::Home),
    Page::PageSortUnsigned(UnsignedPage::SignIn),
    Page::PageSortUnsigned(UnsignedPage::Register)
];

pub const list_of_shared_pages: [Page; 3] = [
    Page::PageSortShared(SharedPage::AboutProject),
    Page::PageSortShared(SharedPage::Docs),
    Page::PageSortShared(SharedPage::PrivacyAndSecurity),
];
