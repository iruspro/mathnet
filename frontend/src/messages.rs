use crate::pages::logged_in_pages::list_of_exercises;
use crate::structs::chat_message::ChatId;
use crate::structs::user::{
    UserChangingProfileData, UserDemandsUserProfileDataChanges, UserId, UserLoginData,
    UserRegisterData, get_user_register_data};
use crate::list_of_pages::{ExerciseId, ListOfExercisesId, Page};

#[derive(Debug, Clone,PartialEq)]
pub enum DefinedMsg {
    SetPage(GoToPage),
    ChangingHash(Page),
    LoginAttempt(UserLoginAttempt),
    Registration(UserRegister),
    UserWantsToChangeProfileData(UserDemandsUserProfileDataChanges),
    SearchFriend(String),
    NoAction,
    UserWantsToChatWithSomePerson(UserId),
    UserWantsToChatWithSomePersonViaPersonalConversation(ChatId),
    SetConversationToNone,
    NoOp,
    SendConversationMessage(String),
}

#[derive(Debug, Clone,PartialEq)]
pub enum GoToPage {
    GoToPageSigned(SwitchToPageSigned),
    GoToPageUnsigned(SwitchToPageUnsigned),
    GoToPageShared(SwitchToPageShared),
    GoToPageOther(SwitchToPageOther),
}

#[derive(Debug, Clone,PartialEq)]
pub enum SwitchToPageUnsigned {
    GoToHomePage,
    GoToLoginPage,
    GoToRegister,
}

#[derive(Debug, Clone,PartialEq)]
pub enum SwitchToPageSigned {
    GoToGroupsList,
    GoToUserProfile(UserId),
    GoToSettings,
    GoToChatWithFriends,
    GoToNotifications,
    GoToExercise(ListOfExercisesId,ExerciseId),
    GoToListOfExercises(ListOfExercisesId),
    GoToGroup(GroupId),
    GoToProfile(UserId),
    GoToUserSuggestsDevelopers,
    GoToFriends,
}
#[derive(Debug, Clone,PartialEq)]
pub enum SwitchToPageShared {
    GoToDocsPage,
    GoToAboutProject,
    GoToPrivacyAndSecurity,
}

#[derive(Debug, Clone,PartialEq)]
pub enum SwitchToPageOther{
    GoToDeleteAccount,
    GoToLogOut,
    GoToRetryChangingUserProfileData,
    GoToSuccessfullyChangedUserProfileData
    
}

#[derive(Debug, Clone,PartialEq)]
pub enum UserLoginAttempt {
    UpdateUserName(String),
    UpdateUserPassword(String),
    CheckLoginValidy,
}

#[derive(Debug, Clone,PartialEq)]
pub enum UserRegister {
    UpdateUserRegisterName(String),
    UpdateUserRegisterPassword(String),
    UpdateUserRegisterPasswordAgain(String),
    UpdateUserRegisterEmail(String),
    RegistrationAttempt,
}
