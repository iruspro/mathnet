use sauron::prelude::*;
use crate::messages::{Msg, GoToPage, SwitchToPageSigned};
use crate::app::App;
use crate::logics::{displaying_friends::{show_friends_at_sidebar,show_chats_in_content}, displaying_conversation};
use crate::experimental_examples::imaginary_friends;
use crate::list_of_pages::{Page,SignedPage};
use sauron::node;

fn show_nav_link(page : SignedPage)->Node<Msg>{
match page{
SignedPage::UserProfile => {
node!{<li class="nav-item">
                        <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile))}>"User profile"</a>
                    </li>
    }
},
SignedPage::ChatWithFriends => {
    node!{<li class="nav-item">
                        <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToChatWithFriends))}>"Chat with friends"</a>
                    </li>
    }
}
SignedPage::GroupsList => {
    node!{<li class="nav-item">
                        <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroupsList))}>"Groups"</a>
                    </li>
    }
}
SignedPage::Docs => {
    node!{<li class="nav-item">
                        <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToDocs))}>"Docs"</a>
                    </li>
    }
},
SignedPage::AboutProject => {
    node!{<li class="nav-item">
                        <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToAboutProject))}>"About this project"</a>
                    </li>
    }
},
SignedPage::PrivacyAndSecurity => {
    node!{<li class="nav-item">
                        <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToPrivacyAndSecurity))}>"Privacy and security"</a>
                    </li>
    }
},
SignedPage::Notifications => {
    node!{<li class="nav-item">
                        <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToNotifications))}>"Notifications"</a>
                    </li>
    }
},
SignedPage::Settings => {
    node!{<li class="nav-item">
                        <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToSettings))}>"Settings"</a>
                    </li>
    }
},
SignedPage::LogOut => {
    node!{<li class="nav-item">
                        <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToLogOut))}>"Log out"</a>
                    </li>
    }
},
_ => {
    node!{
      <li class="nav-item">
                        <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToLogOut))}>"Log out"</a>
                    </li>  
    }
}


}

    

}
fn show_active_nav_link(page_name : String)->Node<Msg>{
    node!{
<li class="nav-item">
                <a class="nav-link text-white" href="#">text!(page_name)</a>
            </li>

}}


/* Handling left sidebar display*/
pub fn left_sidebar(current_page : Page)-> Node<Msg>{
    let list_of_signed_pages :[Page;9]=[
Page::ItemSignedPage(SignedPage::UserProfile),
Page::ItemSignedPage(SignedPage::ChatWithFriends), 
Page::ItemSignedPage(SignedPage::GroupsList),
Page::ItemSignedPage(SignedPage::Docs),
Page::ItemSignedPage(SignedPage::AboutProject),
Page::ItemSignedPage(SignedPage::PrivacyAndSecurity),
Page::ItemSignedPage(SignedPage::Notifications),
Page::ItemSignedPage(SignedPage::Settings),
Page::ItemSignedPage(SignedPage::LogOut),
    ];

}
node!{
            <div class="sidebar d-none d-md-block text-white">
                <h4>"Sidebar"</h4>
                <ul class="nav flex-column">
                    show_nav_link(current_page)
                </ul>
            </div>
}


/* Handling left sidebar display*/
pub fn right_sidebar() -> Node<Msg> {
node!{
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

}}