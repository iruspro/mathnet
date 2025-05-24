/* This file is for routing messages*/
use crate::list_of_pages::{Page, SignedPage, UnsignedPage,SharedPage,OtherPage};
use crate::pages::logged_in_pages::{exercise, list_of_exercises};
use crate::structs::{chat_message::{ChatId}, list_of_exercises::{ListOfExercisesId}, group_struct::{GroupId}, exercise::ExerciseId,user::{UserId}};
use crate::messages::DefinedMsg;
use crate::logics::special_parsing::parse_from_str_to_u32;
use crate::app::App;

impl Page {
    pub fn parse(hash: &str) -> Self{
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
            ["exercise", list_of_exercises_id, exercise_id] => Page::ItemSignedPage(SignedPage::Exercise(ListOfExercisesId::ListOdExercisesIdNumber(parse_from_str_to_u32(list_of_exercises_id)),ExerciseId::ExerciseIdNumber(parse_from_str_to_u32(exercise_id)))),
            ["groups"] => Page::ItemSignedPage(SignedPage::GroupsList),
            ["group", group_id] => Page::ItemSignedPage(SignedPage::Group(GroupId::GroupId(parse_from_str_to_u32(group_id)))),
            ["list_of_exercises", list_of_exercises_id] => Page::ItemSignedPage(SignedPage::ListOfExercises(ListOfExercisesId::ListOdExercisesIdNumber(parse_from_str_to_u32(list_of_exercises_id)))),
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

pub fn routing_page_messages(page_message : DefinedMsg, current_state_of_app: &mut App){
match page_message{
//Unsigned pages
DefinedMsg::SetPage(Page::ItemUnsignedPage(UnsignedPage::Home)) =>{
                current_state_of_app.current_page = Page::ItemUnsignedPage(UnsignedPage::Home)
            },
DefinedMsg::SetPage(Page::ItemUnsignedPage(UnsignedPage::Login))=>{
                current_state_of_app.current_page = Page::ItemUnsignedPage(UnsignedPage::Login)
            },
DefinedMsg::SetPage(Page::ItemUnsignedPage(UnsignedPage::Register))=>{
                current_state_of_app.current_page = Page::ItemUnsignedPage(UnsignedPage::Register)
            },



//Signed pages
DefinedMsg::SetPage(Page::ItemSignedPage(SignedPage::Chat(chat_id))) => {
    current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Chat(chat_id))
},
DefinedMsg::SetPage(Page::ItemSignedPage(SignedPage::UserProfile(user_id))) => {
    current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::UserProfile(user_id))
},
DefinedMsg::SetPage(Page::ItemSignedPage(SignedPage::Settings)) => {
    current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Settings)
},
DefinedMsg::SetPage(Page::ItemSignedPage(SignedPage::Notifications)) => {
    current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Notifications)
},
DefinedMsg::SetPage(Page::ItemSignedPage(SignedPage::ChatWithFriends)) => {
    current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::ChatWithFriends)
},
DefinedMsg::SetPage(Page::ItemSignedPage(SignedPage::Friends)) => {
    current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Friends)
},
DefinedMsg::SetPage(Page::ItemSignedPage(SignedPage::Profile(user_id))) => {
    current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Profile(user_id))
},
DefinedMsg::SetPage(Page::ItemSignedPage(SignedPage::Group(group_id))) => {
    current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Group(group_id))
},
DefinedMsg::SetPage(Page::ItemSignedPage(SignedPage::Exercise(list_of_exercises_id,exercise_id) )) => {
    current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Exercise(list_of_exercises_id,exercise_id))
},
DefinedMsg::SetPage(Page::ItemSignedPage(SignedPage::ListOfExercises(list_of_exercises_id))) => {
    current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::ListOfExercises(list_of_exercises_id))
},





//Shared pages
DefinedMsg::SetPage(Page::ItemSharedPage(SharedPage::AboutProject)) => 
{current_state_of_app.current_page = Page::ItemSharedPage(SharedPage::AboutProject)},
DefinedMsg::SetPage(Page::ItemSharedPage(SharedPage::Docs)) => 
{current_state_of_app.current_page = Page::ItemSharedPage(SharedPage::Docs)},
DefinedMsg::SetPage(Page::ItemSharedPage(SharedPage::PrivacyAndSecurity)) => 
{current_state_of_app.current_page = Page::ItemSharedPage(SharedPage::PrivacyAndSecurity)},


//Other pages
DefinedMsg::SetPage(Page::ItemOtherPage(OtherPage::DeleteAccount)) =>{
    current_state_of_app.current_page = Page::ItemOtherPage(OtherPage::DeleteAccount)
},
DefinedMsg::SetPage(Page::ItemOtherPage(OtherPage::LogOut)) =>{
    current_state_of_app.current_page = Page::ItemOtherPage(OtherPage::LogOut)
},
DefinedMsg::SetPage(Page::ItemOtherPage(OtherPage::SuccessfullyChangedUserProfileData)) =>{
    current_state_of_app.current_page = Page::ItemOtherPage(OtherPage::SuccessfullyChangedUserProfileData)
},
DefinedMsg::SetPage(Page::ItemOtherPage(OtherPage::RetryChangingUserProfileData)) =>{
    current_state_of_app.current_page = Page::ItemOtherPage(OtherPage::RetryChangingUserProfileData)
},
_ => current_state_of_app.current_page = Page::ItemOtherPage(OtherPage::NotFound)

/* Code that is no longer in use

            DefinedMsg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToDocsPage)) => {
                current_state_of_app.current_page = Page::ItemUnsignedPage(UnsignedPage::Docs)
            }
            DefinedMsg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToLoginPage)) => {
                current_state_of_app.current_page = Page::ItemUnsignedPage(UnsignedPage::Login)
            }
            DefinedMsg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToRegister)) => {
                current_state_of_app.current_page = Page::ItemUnsignedPage(UnsignedPage::Register)
            }
            DefinedMsg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToAboutProject)) => {
                current_state_of_app.current_page = Page::ItemUnsignedPage(UnsignedPage::AboutProject)
            }
            DefinedMsg::SetPage(GoToPage::GoToPageUnsigned(
                SwitchToPageUnsigned::GoToPrivacyAndSecurity,
            )) => current_state_of_app.current_page = Page::ItemUnsignedPage(UnsignedPage::PrivacyAndSecurity),

            DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroupsList)) => {
                current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::GroupsList)
            }
            DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToAboutProject)) => {
                current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::AboutProject)
            }
            DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToPrivacyAndSecurity)) => {
                current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::PrivacyAndSecurity)
            }
            DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile)) => {
                current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::UserProfile)
            }
            DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToSettings)) => {
                current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Settings)
            }
            DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToDocsPage)) => {
                current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Docs)
            }
            DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToNotifications)) => {
                current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::Notifications)
            }
            DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToChatWithFriends)) => {
                current_state_of_app.current_page = Page::ItemSignedPage(SignedPage::ChatWithFriends)
            }
            DefinedMsg::SetPage(GoToPage::GoToPageOther(SwitchToPageOther::GoToDeleteAccount)) => {
                current_state_of_app.current_page = Page::ItemOtherPage(OtherPage::DeleteAccount)
            }
            DefinedMsg::SetPage(GoToPage::GoToPageOther(
                SwitchToPageOther::GoToRetryChangingUserProfileData,
            )) => current_state_of_app.current_page = Page::ItemOtherPage(OtherPage::RetryChangingUserProfileData),
            DefinedMsg::SetPage(GoToPage::GoToPageOther(
                SwitchToPageOther::GoToSuccessfullyChangedUserProfileData,
            )) => {
                current_state_of_app.current_page =
                    Page::ItemOtherPage(OtherPage::SuccessfullyChangedUserProfileData)
            }
            DefinedMsg::SetPage(GoToPage::GoToPageOther(SwitchToPageOther::GoToLogOut)) => {
                current_state_of_app.current_page = Page::ItemOtherPage(OtherPage::LogOut)
            }
*/
        }}
