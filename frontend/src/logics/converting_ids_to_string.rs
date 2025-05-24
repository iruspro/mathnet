pub use crate::structs::{user::UserId,exercise::ExerciseId,list_of_exercises::ListOfExercisesId,group_struct::GroupId,chat_message::ChatId};

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
        ListOfExercisesId::ListOdExercisesIdNumber(id) => id.to_string(),
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
