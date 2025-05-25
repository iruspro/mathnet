use crate::messages::{GoToPage,SwitchToPageSigned,SwitchToPageUnsigned,SwitchToPageOther,SwitchToPageShared,DefinedMsg};
use crate::list_of_pages::{SignedPage,UnsignedPage,SharedPage,OtherPage,Page};

pub fn go_to_page_to_page(go_to_page: GoToPage) -> Page {
    match go_to_page {
        GoToPage::GoToPageUnsigned(unsigned) => match unsigned {
            SwitchToPageUnsigned::GoToHomePage => Page::ItemUnsignedPage(UnsignedPage::Home),
            SwitchToPageUnsigned::GoToLoginPage => Page::ItemUnsignedPage(UnsignedPage::Login),
            SwitchToPageUnsigned::GoToRegister => Page::ItemUnsignedPage(UnsignedPage::Register),
        },
        GoToPage::GoToPageSigned(signed) => match signed {
            SwitchToPageSigned::GoToChat(chat_id) => Page::ItemSignedPage(SignedPage::Chat(chat_id)),
            SwitchToPageSigned::GoToGroupsList => Page::ItemSignedPage(SignedPage::GroupsList),
            SwitchToPageSigned::GoToUserProfile(user_id) => Page::ItemSignedPage(SignedPage::UserProfile(user_id)),
            SwitchToPageSigned::GoToSettings => Page::ItemSignedPage(SignedPage::Settings),
            SwitchToPageSigned::GoToNotifications => Page::ItemSignedPage(SignedPage::Notifications),
            SwitchToPageSigned::GoToChatWithFriends => Page::ItemSignedPage(SignedPage::ChatWithFriends),
            SwitchToPageSigned::GoToExercise(list_id, exercise_id) => {
                Page::ItemSignedPage(SignedPage::Exercise(list_id, exercise_id))
            }
            SwitchToPageSigned::GoToListOfExercises(list_id) => {
                Page::ItemSignedPage(SignedPage::ListOfExercises(list_id))
            }
            SwitchToPageSigned::GoToGroup(group_id) => Page::ItemSignedPage(SignedPage::Group(group_id)),
            SwitchToPageSigned::GoToFriends => Page::ItemSignedPage(SignedPage::Friends),
            SwitchToPageSigned::GoToProfile(user_id) => Page::ItemSignedPage(SignedPage::Profile(user_id)),

            // You had this variant but no mapping in Page:
            // SwitchToPageSigned::GoToUserSuggestsDevelopers,
            // Mapping to UserSuggestsToDevelopers in Page enum:
            SwitchToPageSigned::GoToUserSuggestsDevelopers => {
                Page::ItemSignedPage(SignedPage::UserSuggestsToDevelopers)
            }

            // If you add more variants in future, handle here.
        },
        GoToPage::GoToPageOther(other) => match other {
            SwitchToPageOther::GoToDeleteAccount => Page::ItemOtherPage(OtherPage::DeleteAccount),
            SwitchToPageOther::GoToLogOut => Page::ItemOtherPage(OtherPage::LogOut),
            SwitchToPageOther::GoToRetryChangingUserProfileData => {
                Page::ItemOtherPage(OtherPage::RetryChangingUserProfileData)
            }
            SwitchToPageOther::GoToSuccessfullyChangedUserProfileData => {
                Page::ItemOtherPage(OtherPage::SuccessfullyChangedUserProfileData)
            }
        },
        GoToPage::GoToPageShared(shared) => match shared {
            SwitchToPageShared::GoToDocsPage => Page::ItemSharedPage(SharedPage::Docs),
            SwitchToPageShared::GoToAboutProject => Page::ItemSharedPage(SharedPage::AboutProject),
            SwitchToPageShared::GoToPrivacyAndSecurity => Page::ItemSharedPage(SharedPage::PrivacyAndSecurity),
        },
    }
}

pub fn defined_msg_to_page(msg: DefinedMsg) -> Option<Page> {
    match msg {
        DefinedMsg::SetPage(go_to_page) => Some(go_to_page_to_page(go_to_page.clone())),
        _ => None,
    }
}

pub fn page_to_go_to_page(page: Page) -> GoToPage {
    match page {
        Page::ItemUnsignedPage(unsigned) => match unsigned {
            UnsignedPage::Home => GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToHomePage),
            UnsignedPage::Login => GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToLoginPage),
            UnsignedPage::Register => GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToRegister),
        },
        Page::ItemSignedPage(signed) => match signed {
            SignedPage::GroupsList => GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroupsList),
            SignedPage::UserProfile(user_id) => GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile(user_id)),
            SignedPage::Settings => GoToPage::GoToPageSigned(SwitchToPageSigned::GoToSettings),
            SignedPage::Notifications => GoToPage::GoToPageSigned(SwitchToPageSigned::GoToNotifications),
            SignedPage::ChatWithFriends => GoToPage::GoToPageSigned(SwitchToPageSigned::GoToChatWithFriends),
            SignedPage::Exercise(list_id, exercise_id) => GoToPage::GoToPageSigned(SwitchToPageSigned::GoToExercise(list_id, exercise_id)),
            SignedPage::ListOfExercises(list_id) => GoToPage::GoToPageSigned(SwitchToPageSigned::GoToListOfExercises(list_id)),
            SignedPage::Group(group_id) => GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroup(group_id)),
            SignedPage::Friends => GoToPage::GoToPageSigned(SwitchToPageSigned::GoToFriends),
            SignedPage::Profile(user_id) => GoToPage::GoToPageSigned(SwitchToPageSigned::GoToProfile(user_id)),
            SignedPage::UserSuggestsToDevelopers => GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserSuggestsDevelopers),
            SignedPage::Chat(chat_id) => GoToPage::GoToPageSigned(SwitchToPageSigned::GoToChat(chat_id)),
            // Add other variants if any
        },
        Page::ItemOtherPage(other) => match other {
            OtherPage::DeleteAccount => GoToPage::GoToPageOther(SwitchToPageOther::GoToDeleteAccount),
            OtherPage::LogOut => GoToPage::GoToPageOther(SwitchToPageOther::GoToLogOut),
            OtherPage::RetryChangingUserProfileData => GoToPage::GoToPageOther(SwitchToPageOther::GoToRetryChangingUserProfileData),
            OtherPage::SuccessfullyChangedUserProfileData => GoToPage::GoToPageOther(SwitchToPageOther::GoToSuccessfullyChangedUserProfileData),
            OtherPage::NotFound => {
                // No direct equivalent in GoToPage; fallback:
                GoToPage::GoToPageOther(SwitchToPageOther::GoToLogOut) // or create a NotFound variant if needed
            }
        },
        Page::ItemSharedPage(shared) => match shared {
            SharedPage::Docs => GoToPage::GoToPageShared(SwitchToPageShared::GoToDocsPage),
            SharedPage::AboutProject => GoToPage::GoToPageShared(SwitchToPageShared::GoToAboutProject),
            SharedPage::PrivacyAndSecurity => GoToPage::GoToPageShared(SwitchToPageShared::GoToPrivacyAndSecurity),
        },
    }
}

