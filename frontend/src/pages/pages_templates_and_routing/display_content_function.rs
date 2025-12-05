use crate::app::Msg;
type MSG = Msg;
use crate::list_of_pages::{OtherPage, Page};
use crate::pages::delete_account::delete_account_display;
use crate::pages::{chat_with_friends, mathnet_notifications, privacy_and_security};
use crate::pages::{
    about_project::about_project_display,
    base_of_exercises::base_of_exercises_display(),
    chat_with_friends::chat_with_friends_display(),
    conversation::conversation_display(),
    delete_account::delete_account_display(),
    docs::docs_display(),
    exercise::exercise_display(),
    groups::group_display(),
    home::home_display(),
    learning_resources::learning_resources_display(),
    mathnet_notifications::mathnet_notifications_display(),
    privacy_and_security::privacy_and_security_display(),
    register::register_display(),
    settings::settings_display(),
    sign_in::sign_in_display();
    sign_out::sign_out_display,
    user_profile::user_profile_display,




}
use sauron::prelude::*;

pub fn display_content(page: &Page) -> Node<MSG>{
match page {
    Page::PageSortSigned(SignedPage::BaseOfExercises) => about_project_display(),
    Page::PageSortSigned(SignedPage::BaseOfLearningResources) => base_of_exercises_display(),
    Page::PageSortSigned(SignedPage::ChatWithFriends) => chat_with_friends_display(),
    Page::PageSortSigned(SignedPage::Conversation) => conversation_display(),
    Page::PageSortSigned(SignedPage::Exercise) => exercise_display(),
    Page::PageSortSigned(SignedPage::Group) => group_display(),
    Page::PageSortSigned(SignedPage::GroupList) => group_list_display(),
    Page::PageSortSigned(SignedPage::LearningResources) => learning_resources_display(),
    Page::PageSortSigned(SignedPage::MathNetNotifications) => mathnet_notifications_display(),
    Page::PageSortSigned(SignedPage::Settings) => settings_display(),
    Page::PageSortSigned(SignedPage::UserProfile) => user_profile_display(),

    Page::PageSortUnsigned(UnsignedPage::Home) => home_display(),
    Page::PageSortUnsigned(UnsignedPage::SignIn) => sign_in_display(),
    Page::PageSortUnsigned(UnsignedPage::Register)=> register_display(),

    Page::PageSortShared(SharedPage::AboutProject) => about_project_display(),
    Page::PageSortShared(SharedPage::Docs) => docs_display(),
    Page::PageSortShared(SharedPage::PrivacyAndSecurity) => privacy_and_security_display(),

    Page::PageSortOther(OtherPage::DeleteAccount) => delete_account_display(),
    Page::PageSortOther(OtherPage::SignOut) => sign_out_display(),














    
}
}