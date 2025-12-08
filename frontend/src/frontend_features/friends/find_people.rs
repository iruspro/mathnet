/*
Find people in mathnet database and show targets. Show results in special window inside conversation_list_display.
*/

use sauron::prelude::*;
use sauron::InputEvent;
use crate::{list_of_pages::{SignedPage, Page}};
use crate::{messages::Msg};


#[derive(Clone, PartialEq, Debug)]
enum PersonQueryStatus{
    NotYourFriend,
    FriendRequestSent,
    YourFriend
}

#[derive(Clone, PartialEq, Debug)]
pub struct PersonSearchQuery{
    name: String,
    user_id: u128, 
    person_query_status: PersonQueryStatus
}

pub fn find_person() -> Node<Msg>{
    node!{<form>
            <textarea
            id="person_search_input"
                placeholder="Find people you want!"
                on_input=|input: InputEvent| {
                    Msg::UpdatePersonSearch(input.value())
                }></textarea>

            <button on_click= move |_| { Msg::SearchFriend }>"Find"</button>
        <label for="person_search_input">"Find people"</label>
        </form>
}}

pub fn search_results(search_results: Vec<PersonSearchQuery>) -> Node<Msg> {
    match search_results {
       v if v == Vec::new() => {
        node!{
            <div class="scrollable-box">
                "No results found."
            </div>
            
        
       }}
       _ =>{
            node!{
                <div class="scrollable-box">
                    {for result in search_results{
                        person_search_result_box(result)
                    }}
                </div>
            }}
    }
}

fn person_search_result_box(search_result: PersonSearchQuery) -> Node<Msg> {
    match search_result.person_query_status {
        PersonQueryStatus::NotYourFriend => {
            node!{
                <div class="single_friend_search_box">
                text!("{}",search_result.name)
                <button on_click=|_|{Msg::SendFriendRequest}> "Send friend request" </button>
                </div>
            }
        }
        PersonQueryStatus::FriendRequestSent => {
            node!{
                <div class="single_friend_search_box">
                text!("{}",search_result.name)
                <button on_click=|_|{Msg::UnsendFriendRequest}> "Unsend friend request" </button>
                </div>
            }
        }
        PersonQueryStatus::YourFriend => {
            node!{<div class="single_friend_search_box">
                text!("{}",search_result.name)
                <button on_click=|_|{Msg::SetPage(Page::PageSortSigned(SignedPage::Conversation))}> "Chat with them!" </button> // Conversation page hasn't been prepared yet.
                </div>
            }
        }
    }


}

/*
Since search result mechanism will be similar at events, groups, 
projects etc., it probably makes sense to write a special trait for this kind 
of behaviour.
*/