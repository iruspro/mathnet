/*
Find people in mathnet database and show targets. Show results in special window inside conversation_list_display.
*/

use sauron::prelude::*;
use sauron::InputEvent;
use crate::{list_of_pages::{SignedPage, Page}};
use crate::{messages::Msg};


#[derive(Clone, PartialEq, Debug)]
enum FriendQueryStatus{
    YourFriend,
    Blocked,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FriendSearchQuery{
    name: String,
    user_id: u128, 
    friend_query_status: FriendQueryStatus,
}


pub fn find_friend() -> Node<Msg>{
    node!{<form>
            <textarea
            id="friend_search_input"
                placeholder="Find your friends"
                on_input=|input: InputEvent| {Msg::UpdateFriendSearch(input.value())}></textarea>

            <button on_click= move |_| { Msg::SearchFriend }>"Find"</button>
        <label for="friend_search_input">"Find people"</label>
        </form>
}}

// Meant for getting search results from database.
pub fn search_results(search_results: Vec<FriendSearchQuery>) -> Node<Msg> {
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
                        friend_search_result_box(result)
                    }}
                </div>
            }}
    }
}

fn friend_search_result_box(search_result: FriendSearchQuery) -> Node<Msg> {
    match search_result.friend_query_status {
        FriendQueryStatus::YourFriend => {
            node!{
                <div class="single_friend_search_box">
                text!("{}",search_result.name)
                <button on_click=|_|{Msg::SetPage(Page::PageSortSigned(SignedPage::Conversation))}> "Chat" </button>
                <button on_click=|_|{Msg::BlockFriend}>"Block"</button>
                <button on_click=|_|{Msg::RemoveFriend}>"Remove"</button>
                </div>
            }
        }
        FriendQueryStatus::Blocked => {
            node!{
                <div class="single-friend-search-box">
                text!("{}",search_result.name)
                <button on_click=|_|{Msg::UnblockFriend}> "Unblock" </button>
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