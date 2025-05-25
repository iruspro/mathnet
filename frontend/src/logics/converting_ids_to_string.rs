pub use crate::structs::{user::UserId,exercise::ExerciseId,list_of_exercises::ListOfExercisesId,group_struct::GroupId,chat_message::ChatId};
pub use crate::messages::{};
pub use crate::list_of_pages::{Page, UnsignedPage,SignedPage,SharedPage,OtherPage};

pub fn user_id_to_string(user_id: UserId) -> String {
    match user_id {
        UserId::UserIdNumber(id) => id.to_string(),
    }
}

pub fn exercise_id_to_string(ex_id: ExerciseId) -> String {
    match ex_id {
        ExerciseId::ExerciseIdNumber(id) => id.to_string(),
    }
}

pub fn list_of_exercises_id_to_string(list_id: ListOfExercisesId) -> String {
    match list_id {
        ListOfExercisesId::ListOfExercisesIdNumber(id) => id.to_string(),
    }
}

pub fn group_id_to_string(group_id: GroupId) -> String {
    match group_id {
        GroupId::GroupIdNumber(id) => id.to_string(),
    }
}

pub fn chat_id_to_string(chat_id: ChatId) -> String {
    match chat_id {
        ChatId::ChatIdNumber(id) => id.to_string(),
    }
}


pub fn string_to_page(s: String) -> Page {
    match s.as_str() {
        "Home" => Page::ItemUnsignedPage(UnsignedPage::Home),
"Login" => Page::ItemUnsignedPage(UnsignedPage::Login),
"Register" => Page::ItemUnsignedPage(UnsignedPage::Register),

"Groups_list" => Page::ItemSignedPage(SignedPage::GroupsList),
"User_profile" => Page::ItemSignedPage(SignedPage::UserProfile(UserId::UserIdNumber(1))),
"Settings" => Page::ItemSignedPage(SignedPage::Settings),
"Notifications" => Page::ItemSignedPage(SignedPage::Notifications),
"Chat_with_friends" => Page::ItemSignedPage(SignedPage::ChatWithFriends),
"Chat" => Page::ItemSignedPage(SignedPage::Chat(ChatId::ChatIdNumber(1))),
"Friends" => Page::ItemSignedPage(SignedPage::Friends),
"Profile" => Page::ItemSignedPage(SignedPage::Profile(UserId::UserIdNumber(1))),
"Exercise" => Page::ItemSignedPage(SignedPage::Exercise(ListOfExercisesId::ListOfExercisesIdNumber(1), ExerciseId::ExerciseIdNumber(1))),
"List_of_exercises" => Page::ItemSignedPage(SignedPage::ListOfExercises(ListOfExercisesId::ListOfExercisesIdNumber(1))),
"Group" => Page::ItemSignedPage(SignedPage::Group(GroupId::GroupIdNumber(1))),
"Suggest_to_the_developers" => Page::ItemSignedPage(SignedPage::UserSuggestsToDevelopers),

"Logout" => Page::ItemOtherPage(OtherPage::LogOut),
"Delete_account" => Page::ItemOtherPage(OtherPage::DeleteAccount),
"Successfully_changed_user_profile_data" => Page::ItemOtherPage(OtherPage::SuccessfullyChangedUserProfileData),
"Retry_changing_user_profile_data" => Page::ItemOtherPage(OtherPage::RetryChangingUserProfileData),
"Page_not_found" => Page::ItemOtherPage(OtherPage::NotFound),

"Docs" => Page::ItemSharedPage(SharedPage::Docs),
"About_this_project" => Page::ItemSharedPage(SharedPage::AboutProject),
"Privacy_and_security" => Page::ItemSharedPage(SharedPage::PrivacyAndSecurity),

        _ => Page::ItemUnsignedPage(UnsignedPage::Home),  // or other default
    }
}

