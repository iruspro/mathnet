use crate::app::App;
use crate::experimental_examples::imaginary_friends;
use crate::logics::{
    displaying_conversation,
    displaying_friends::{show_chats_in_content, show_friends_at_sidebar},
    sidebars,
};
use crate::messages::{GoToPage, Msg, SwitchToPageOther, SwitchToPageSigned};
use sauron::prelude::*;

pub fn view(data_provided: &App) -> Node<Msg> {
    node! {
        <main>
            // Left Sidebar (Desktop)
            {sidebars::left_sidebar(data_provided.current_page.clone())}

            // Right Sidebar
            <div class="right-sidebar d-none d-md-block text-white">
                <h4>"Right Sidebar"</h4>
                <ul class="nav flex-column">
                <div class="search-container">
        <input type="text" class="form-control search-input" placeholder="Find a person" on_input=|input|{Msg::SearchFriend(input.value())}></input>
        <i class="fas fa-search search-icon"></i>
      </div>
                    //{show_friends_at_sidebar()};
                </ul>
            </div>

            // Main Content
            <div class="content">
                <div class="scrollable-container"> //scrollable-container doesn't do anything because sidebars are already fixed (independent from scrolling the content.)
                    <h1 class="text-center">"Find friends"</h1>
                    <p class="basicparagraph text-start">
                        "View other people's profiles or chat with somebody and have fun!"
                        {for _ in (0..100){show_chats_in_content(vec!(
                            imaginary_friends::dummy_show_chat(),
                            ))
                        }}
                    </p>
                </div>
            </div>
        </main>
    }
}
