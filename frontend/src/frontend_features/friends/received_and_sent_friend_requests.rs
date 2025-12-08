/*
Show received and sent friend requests
*/

use sauron::prelude::*;
use sauron::InputEvent;
use crate::{list_of_pages::{SignedPage, Page}};
use crate::{messages::Msg};

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
enum ReceivedOrSent{
    Received,
    Sent
}

#[derive(Clone, PartialEq, Debug)]
struct FriendRequest{
    friend_request_status: ReceivedOrSent,
    person_name: String,
    person_id: u128

}

fn sort_by_friend_request_status(friend_requests_list: Vec<FriendRequest>) -> Vec<FriendRequest>{
    match friend_requests_list {
        v if v == Vec::new() => {v}
        frl => {
            let mut frl_cloned = frl.clone();
            frl_cloned.sort_by_key(|req| req.friend_request_status.clone());
            return frl_cloned

        }

    }
}

// Meant for getting search results from database.
pub fn search_results(search_results: Vec<FriendRequest>) -> Node<Msg> {
    let search_results = sort_by_friend_request_status(search_results);
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

fn friend_request_box(search_result: FriendRequest) -> Node<Msg> {
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
                <div class="single_friend_search_box">
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
