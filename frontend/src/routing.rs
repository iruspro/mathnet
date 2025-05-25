/* This file is for routing messages*/
use crate::list_of_pages::{Page, SignedPage, UnsignedPage,SharedPage,OtherPage};
use crate::pages::logged_in_pages::{exercise, list_of_exercises};
use crate::structs::{chat_message::{ChatId}, list_of_exercises::{ListOfExercisesId}, group_struct::{GroupId}, exercise::ExerciseId,user::{UserId}};
use crate::messages::{DefinedMsg, GoToPage, SwitchToPageSigned,SwitchToPageUnsigned,SwitchToPageShared,SwitchToPageOther};
use crate::logics::special_parsing::parse_from_str_to_u32;
use crate::app::App;
use crate::logics::converting_ids_to_string::{chat_id_to_string,group_id_to_string,exercise_id_to_string,list_of_exercises_id_to_string,user_id_to_string};
use crate::web_sys;
use web_sys::console;


impl Page {
pub fn parse_hashes(hash: &str) -> DefinedMsg {
    let hash = hash.trim_start_matches('#').trim_start_matches('/');
    let segments: Vec<&str> = hash.split('/').collect();
    match segments.as_slice() {
        // logout pages (unsigned)
        ["home"] => DefinedMsg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToHomePage)),
        ["login"] => DefinedMsg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToLoginPage)),
        ["register"] => DefinedMsg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToRegister)),

        // login pages (signed)
        ["chat_with_friends"] => DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToChatWithFriends)),
        ["chat", chat_id] => DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToChat(ChatId::ChatIdNumber(parse_from_str_to_u32(chat_id))))),
        ["exercise", list_of_exercises_id, exercise_id] => DefinedMsg::SetPage(GoToPage::GoToPageSigned(
            SwitchToPageSigned::GoToExercise(
                ListOfExercisesId::ListOfExercisesIdNumber(parse_from_str_to_u32(list_of_exercises_id)),
                ExerciseId::ExerciseIdNumber(parse_from_str_to_u32(exercise_id))
            )
        )),
        ["groups"] => DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroupsList)),
        ["group", group_id] => DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroup(GroupId::GroupIdNumber(parse_from_str_to_u32(group_id))))),
        ["list_of_exercises", list_of_exercises_id] => DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToListOfExercises(ListOfExercisesId::ListOfExercisesIdNumber(parse_from_str_to_u32(list_of_exercises_id))))),
        ["notifications"] => DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToNotifications)),
        ["settings"] => DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToSettings)),
        ["user_profile", user_id] => DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile(UserId::UserIdNumber(parse_from_str_to_u32(user_id))))),
        ["friends"] => DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToFriends)),
        ["profile", user_id] => DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToProfile(UserId::UserIdNumber(parse_from_str_to_u32(user_id))))),

        // other pages
        ["delete_account"] => DefinedMsg::SetPage(GoToPage::GoToPageOther(SwitchToPageOther::GoToDeleteAccount)),
        ["logout"] => DefinedMsg::SetPage(GoToPage::GoToPageOther(SwitchToPageOther::GoToLogOut)), 
        ["retry_changing_user_profile_data"] => DefinedMsg::SetPage(GoToPage::GoToPageOther(SwitchToPageOther::GoToRetryChangingUserProfileData)),
        ["successfully_changed_user_profile_data"] => DefinedMsg::SetPage(GoToPage::GoToPageOther(SwitchToPageOther::GoToSuccessfullyChangedUserProfileData)),

        // shared pages
        ["privacy_and_security"] => DefinedMsg::SetPage(GoToPage::GoToPageShared(SwitchToPageShared::GoToPrivacyAndSecurity)),
        ["docs"] => DefinedMsg::SetPage(GoToPage::GoToPageShared(SwitchToPageShared::GoToDocsPage)),
        ["about_project"] => DefinedMsg::SetPage(GoToPage::GoToPageShared(SwitchToPageShared::GoToAboutProject)),

        _ => DefinedMsg::SetPage(GoToPage::GoToPageOther(SwitchToPageOther::GoToLogOut)), // Or some default fallback, e.g. NotFound page missing here in GoToPageOther
    }
}

    
    pub fn parse(hash: &str) -> Page{
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
            ["exercise", list_of_exercises_id, exercise_id] => Page::ItemSignedPage(SignedPage::Exercise(ListOfExercisesId::ListOfExercisesIdNumber(parse_from_str_to_u32(list_of_exercises_id)),ExerciseId::ExerciseIdNumber(parse_from_str_to_u32(exercise_id)))),
            ["groups"] => Page::ItemSignedPage(SignedPage::GroupsList),
            ["group", group_id] => Page::ItemSignedPage(SignedPage::Group(GroupId::GroupIdNumber(parse_from_str_to_u32(group_id)))),
            ["list_of_exercises", list_of_exercises_id] => Page::ItemSignedPage(SignedPage::ListOfExercises(ListOfExercisesId::ListOfExercisesIdNumber(parse_from_str_to_u32(list_of_exercises_id)))),
            ["notifications"] =>Page::ItemSignedPage(SignedPage::Notifications),
            ["settings"] => Page::ItemSignedPage(SignedPage::Settings),
            ["user_profile", user_id] => Page::ItemSignedPage(SignedPage::UserProfile(UserId::UserIdNumber(parse_from_str_to_u32(user_id)))),
            ["friends"] => Page::ItemSignedPage(SignedPage::Friends),
            ["profile", user_id] => Page::ItemSignedPage(SignedPage::Profile(UserId::UserIdNumber(parse_from_str_to_u32(user_id)))),

            // other pages
            ["delete_account"] => Page::ItemOtherPage(OtherPage::DeleteAccount),
            ["logout"] => Page::ItemOtherPage(OtherPage::LogOut), 
            ["retry_changing_user_profile_data"] => Page::ItemOtherPage(OtherPage::RetryChangingUserProfileData),
            ["successfully_changed_user_profile_data"] => Page::ItemOtherPage(OtherPage::SuccessfullyChangedUserProfileData),


            // shared pages
            ["privacy_and_security"] => Page::ItemSharedPage(SharedPage::PrivacyAndSecurity),
            ["docs"] => Page::ItemSharedPage(SharedPage::Docs),
            ["about_project"] => Page::ItemSharedPage(SharedPage::AboutProject),
            _ => Page::ItemOtherPage(OtherPage::NotFound)

        }
    }

    pub fn to_hash(current_state_of_app:&Page) -> String {
        match current_state_of_app {
            //Unsigned pages
            Page::ItemUnsignedPage(UnsignedPage::Home) => "#/home".into(),
            Page::ItemUnsignedPage(UnsignedPage::Login) => "#/login".into(),
            Page::ItemUnsignedPage(UnsignedPage::Register) => "#/register".into(),

            //Signed pages
            Page::ItemSignedPage(SignedPage::GroupsList) => "#/groups_list".into(),
            Page::ItemSignedPage(SignedPage::UserProfile(user_id)) => format!("#/user_profile/{:?}", user_id.clone()),
            Page::ItemSignedPage(SignedPage::Settings) => "#/settings".into(),
            Page::ItemSignedPage(SignedPage::Notifications) => "#/notifications".into(),
            Page::ItemSignedPage(SignedPage::ChatWithFriends) => "#/chat_with_friends".into(),
            Page::ItemSignedPage(SignedPage::Chat(chat_id)) => format!("#/chat/{:?}", chat_id.clone()),
            Page::ItemSignedPage(SignedPage::Friends) => "#/friends".into(),
            Page::ItemSignedPage(SignedPage::Profile(user_id)) => format!("#/profile/{:?}", user_id.clone()),
            Page::ItemSignedPage(SignedPage::Group(group_id)) => format!("#/groups_list/group/{:?}", group_id.clone()),
            Page::ItemSignedPage(SignedPage::Exercise(list_of_exercises_id, exercise_id)) => format!("#/list_of_exercises/{:?}/exercise/{:?}", list_of_exercises_id.clone(),exercise_id.clone()),
            Page::ItemSignedPage(SignedPage::ListOfExercises(list_of_exercise_id)) => format!("#/list_of_exercises/{:?}", list_of_exercise_id.clone()),
           
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
                "#/privacy_and_security".into()
        },
        _ => "#/404".into()
        }
    }

}



pub fn routing_page_messages(page_message: DefinedMsg, current_state_of_app: &mut App) {
    match page_message {
        DefinedMsg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToHomePage)) => {
            current_state_of_app.current_page = Page::ItemUnsignedPage(UnsignedPage::Home);
        }
        DefinedMsg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToLoginPage)) => {
            current_state_of_app.current_page = Page::ItemUnsignedPage(UnsignedPage::Login);
        }
        DefinedMsg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToRegister)) => {
            current_state_of_app.current_page = Page::ItemUnsignedPage(UnsignedPage::Register);
        }

        DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroupsList)) => {
            current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::GroupsList);
        }
        DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile(user_id))) => {
            current_state_of_app.current_page =
                Page::ItemSignedPage(SignedPage::UserProfile(user_id));
        }
        DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToSettings)) => {
            current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Settings);
        }
        DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToChatWithFriends)) => {
            current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::ChatWithFriends);
        }
        DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToNotifications)) => {
            current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Notifications);
        }
        DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToExercise(list_of_exercises_id,exercise_id))) => {
            current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Exercise(list_of_exercises_id, exercise_id));
        }
        DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToListOfExercises(list_of_exercises_id))) => {
            current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::ListOfExercises(list_of_exercises_id));
        }
        DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroup(group_id))) => {
            current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Group(group_id));
        }
        DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToProfile(user_id))) => {
            current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Profile(user_id));
        }
        DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserSuggestsDevelopers)) => {
            current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::UserSuggestsToDevelopers);
        }
        DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToFriends)) => {
            current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Friends);
        }

        DefinedMsg::SetPage(GoToPage::GoToPageShared(SwitchToPageShared::GoToAboutProject)) => {
            current_state_of_app.current_page = Page::ItemSharedPage(SharedPage::AboutProject);
        }
        DefinedMsg::SetPage(GoToPage::GoToPageShared(SwitchToPageShared::GoToDocsPage)) => {
            current_state_of_app.current_page = Page::ItemSharedPage(SharedPage::Docs);
        }
        DefinedMsg::SetPage(GoToPage::GoToPageShared(SwitchToPageShared::GoToPrivacyAndSecurity)) => {
            current_state_of_app.current_page = Page::ItemSharedPage(SharedPage::PrivacyAndSecurity);
        }

        DefinedMsg::SetPage(GoToPage::GoToPageOther(SwitchToPageOther::GoToDeleteAccount)) => {
            current_state_of_app.current_page = Page::ItemOtherPage(OtherPage::DeleteAccount);
        }
        DefinedMsg::SetPage(GoToPage::GoToPageOther(SwitchToPageOther::GoToLogOut)) => {
            current_state_of_app.current_page = Page::ItemOtherPage(OtherPage::LogOut);
        }
        DefinedMsg::SetPage(GoToPage::GoToPageOther(SwitchToPageOther::GoToRetryChangingUserProfileData)) => {
            current_state_of_app.current_page = Page::ItemOtherPage(OtherPage::RetryChangingUserProfileData);
        }
        DefinedMsg::SetPage(GoToPage::GoToPageOther(SwitchToPageOther::GoToSuccessfullyChangedUserProfileData)) => {
            current_state_of_app.current_page = Page::ItemOtherPage(OtherPage::SuccessfullyChangedUserProfileData);
        }

        // Any other variant of DefinedMsg is ignored
        _ => {}
    }
}


fn go_to_page_to_hash_convertor(go_to_page: GoToPage) -> String {
    match go_to_page {
        GoToPage::GoToPageSigned(page) => match page {
            SwitchToPageSigned::GoToGroupsList => "#groups".to_string(),
            SwitchToPageSigned::GoToUserProfile(user_id) => format!("#user_profile/{}", user_id_to_string(user_id)),
            SwitchToPageSigned::GoToSettings => "#settings".to_string(),
            SwitchToPageSigned::GoToChatWithFriends => "#chat".to_string(),
            SwitchToPageSigned::GoToNotifications => "#notifications".to_string(),
            SwitchToPageSigned::GoToExercise(list_id, ex_id) => format!("#list_of_exercises/{:?}/exercise/{:?}", list_of_exercises_id_to_string(list_id), exercise_id_to_string(ex_id)),
            SwitchToPageSigned::GoToListOfExercises(list_id) => format!("#list_of_exercises/{}", list_of_exercises_id_to_string(list_id)),
            SwitchToPageSigned::GoToGroup(group_id) => format!("#group/{}", group_id_to_string(group_id)),
            SwitchToPageSigned::GoToProfile(user_id) => format!("#profile/{}", user_id_to_string(user_id)),
            SwitchToPageSigned::GoToUserSuggestsDevelopers => "#suggest_developers".to_string(),
            SwitchToPageSigned::GoToFriends => "#friends".to_string(),
            SwitchToPageSigned::GoToChat(chat_id) => format!{"#chat/{:?}", chat_id},
        },
        GoToPage::GoToPageUnsigned(page) => match page {
            SwitchToPageUnsigned::GoToHomePage => "#home".to_string(),
            SwitchToPageUnsigned::GoToLoginPage => "#login".to_string(),
            SwitchToPageUnsigned::GoToRegister => "#register".to_string(),
        },
        GoToPage::GoToPageShared(page) => match page {
            SwitchToPageShared::GoToDocsPage => "#docs".to_string(),
            SwitchToPageShared::GoToAboutProject => "#about".to_string(),
            SwitchToPageShared::GoToPrivacyAndSecurity => "#privacy".to_string(),
        },
        GoToPage::GoToPageOther(page) => match page {
            SwitchToPageOther::GoToDeleteAccount => "#delete_account".to_string(),
            SwitchToPageOther::GoToLogOut => "#logout".to_string(),
            SwitchToPageOther::GoToRetryChangingUserProfileData => "#retry_profile_update".to_string(),
            SwitchToPageOther::GoToSuccessfullyChangedUserProfileData => "#profile_update_success".to_string(),
        },
    }
}


pub fn change_hash_while_navigating(go_to_page: GoToPage){
let new_hash = go_to_page_to_hash_convertor(go_to_page.clone());

            // 2. Update the browser hash:
            if let Some(win) = web_sys::window() {
                let _ = win.location().set_hash(&new_hash);
            }}