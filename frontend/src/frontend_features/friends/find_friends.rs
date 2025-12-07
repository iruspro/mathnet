/*
Find friends in mathnet database and show targets. Show results in special window inside conversation_list_display.
*/

use sauron::prelude::*;
use sauron::InputEvent;
use crate::{messages::Msg, structs::friends_at_sidebar};

fn assign(inp: InputEvent, friend_search_box_input: &mut String){
    friend_search_box_input = inp.value().clone();
}
pub fn find_friends() -> Node<Msg>{
    let mut friend_search_box_input: String = String::new() ;
    node!{
        <form>
        <button on_click =  move |_|{Msg::SearchFriend(friend_search_box_input.clone())}> "Find"</button>
        <input 
        type="text"
        placeholder="Find your friend!"
        // FIX: don't know what is wrong with this.
        on_input= |input|{friend_search_box_input = input.value(); Msg::NoAction}>
         </input>
        </form>
    }

}

