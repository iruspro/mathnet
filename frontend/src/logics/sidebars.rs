use crate::app::App;
use crate::experimental_examples::imaginary_friends;
use crate::list_of_pages::{OtherPage, Page, SignedPage};
use crate::logics::{
    displaying_conversation,
    displaying_friends::{show_chats_in_content, show_friends_at_sidebar},
};
use crate::messages::GoToPage::GoToPageSigned;
use crate::messages::{GoToPage, Msg, SwitchToPageOther, SwitchToPageSigned};
use crate::web_sys::console;
use sauron::node;
use sauron::prelude::*;

fn show_nav_link(page: Page) -> Node<Msg> {
    let pagex = Page::ItemSignedPage(SignedPage::GroupsList);
    console::log_1(&Page::page_name_to_string(page.clone()).into());
    match page {
        Page::ItemSignedPage(SignedPage::UserProfile) => {
            console::log_1(&"Hello from Rust3!".into());
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile))}>"User profile"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::ChatWithFriends) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToChatWithFriends))}>"Chat with friends"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::GroupsList) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroupsList))}>"Groups"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::Docs) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToDocsPage))}>"Docs"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::AboutProject) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToAboutProject))}>"About this project"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::PrivacyAndSecurity) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToPrivacyAndSecurity))}>"Privacy and security"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::Notifications) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToNotifications))}>"Notifications"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::Settings) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToSettings))}>"Settings"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::LogOut) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{Msg::SetPage(GoToPage::GoToPageOther(SwitchToPageOther::GoToLogOut))}>"Log out"</a>
                            </li>
            }
        }
        _ => unimplemented!("Dummy mistake at show_nav_link"),
    }
}

fn show_active_nav_link(page_name: String) -> Node<Msg> {
    if page_name == "Groups list" {
        node! {
        <li class="nav-item">
                        <a class="nav-link text-white" arria-current="page" href="#">{text("Groups".to_string())}</a>
                    </li>
            }
    } else {
        if page_name == "Conversation" {
            node! {
            <li class="nav-item">
                            <a class="nav-link text-white" arria-current = "page" href="#">"Chat with friends"</a>
                        </li>}
        } else {
            node! {
            <li class="nav-item">
                            <a class="nav-link text-white" arria-current = "page" href="#">{text(page_name)}</a>
                        </li>

            }
        }
    }
}

//pub fn left_sidebar(current_page: Page) -> Node<Msg> {
//    let list_of_signed_pages: [Page; 9] = [
//        Page::ItemSignedPage(SignedPage::UserProfile),
//        Page::ItemSignedPage(SignedPage::ChatWithFriends),
//        Page::ItemSignedPage(SignedPage::GroupsList),
//        Page::ItemSignedPage(SignedPage::Docs),
//        Page::ItemSignedPage(SignedPage::AboutProject),
//        Page::ItemSignedPage(SignedPage::PrivacyAndSecurity),
//        Page::ItemSignedPage(SignedPage::Notifications),
//        Page::ItemSignedPage(SignedPage::Settings),
//        Page::ItemSignedPage(SignedPage::LogOut),
//    ];
//
//    // Generate list items outside the node! macro
//    let nav_links: Vec<Node<Msg>> = list_of_signed_pages
//        .iter()
//        .map(|pagex| {
//            if *pagex == current_page {
//                // Extract readable name from current_page for display
//                let name = match pagex {
//                    Page::ItemSignedPage(sp) => format!("{:?}", sp), // Or a custom name function
//                    _ => "Unknown".to_string(),
//                };
//                show_active_nav_link(name)
//            } else {
//                match pagex {
//                    Page::ItemSignedPage(sp) => show_nav_link(*sp),
//                    _ => text!(""), // fallback
//                }
//            }
//        })
//        .collect();
//
//    // Now build the sidebar using the pre-collected nav_links
//    node! {
//        <div class="sidebar d-none d-md-block text-white">
//            <h4>"Sidebar"</h4>
//            <ul class="nav flex-column">
//                {nav_links}
//            </ul>
//        </div>
//    }
//}

/* Handling left sidebar display*/

/*
pub fn left_sidebar(current_page : Page) -> Node<Msg>{
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
node!{
for pagex in list_of_signed_pages{
if pagex == current_page{
    node!{
            <div class="sidebar d-none d-md-block text-white">
                <h4>"Sidebar"</h4>
                <ul class="nav flex-column">
                    show_active_nav_link(current_page)
                </ul>
            </div>
}
}
else {
node!{
            <div class="sidebar d-none d-md-block text-white">
                <h4>"Sidebar"</h4>
                <ul class="nav flex-column">
                    show_nav_link(current_page)
                </ul>
            </div>
}}}}}
*/

/* Handling left sidebar display*/
//pub fn right_sidebar() -> Node<Msg> {
//node!{
//    <div class="right-sidebar d-none d-md-block text-white">
//                <h4>"Right Sidebar"</h4>
//                <ul class="nav flex-column">
//                <div class="search-container">
//        <input type="text" class="form-control search-input" placeholder="Find a person" on_input=|input|{Msg::SearchFriend(input.value())}></input>
//        <i class="fas fa-search search-icon"></i>
//      </div>
//                    //{show_friends_at_sidebar()};
//                </ul>
//            </div>
//
//}}}

use sauron::prelude::*;

// Assume Msg and Page/SignedPage types are already defined

pub fn left_sidebar(current_page: Page) -> Node<Msg> {
    let list_of_signed_pages: [Page; 9] = [
        Page::ItemSignedPage(SignedPage::UserProfile),
        Page::ItemSignedPage(SignedPage::ChatWithFriends),
        Page::ItemSignedPage(SignedPage::GroupsList),
        Page::ItemSignedPage(SignedPage::Docs),
        Page::ItemSignedPage(SignedPage::AboutProject),
        Page::ItemSignedPage(SignedPage::PrivacyAndSecurity),
        Page::ItemSignedPage(SignedPage::Notifications),
        Page::ItemSignedPage(SignedPage::Settings),
        Page::ItemSignedPage(SignedPage::LogOut),
        // let pagex = Page::ItemSignedPage(SignedPage::GroupsList);
    ];

    // Generate nav links as a Vec<Node<Msg>>
    let nav_links: Vec<Node<Msg>> = list_of_signed_pages
        .iter()
        .map(|pagex| {
            if *pagex == current_page {
                console::log_1(&"Rust5".into());
                console::log_1(&Page::page_name_to_string(pagex.clone()).into());
                show_active_nav_link(Page::page_name_to_string(pagex.clone()))
            } else {
                console::log_1(&Page::page_name_to_string(pagex.clone()).into());
                show_nav_link(pagex.clone())
            }
        })
        .collect();

    node! {
        <div class="sidebar d-none d-md-block">
            <h4>"Sidebar"</h4>
            <ul class="nav flex-column">
                {for el in nav_links{
                    el
                }}
            </ul>
        </div>
    }
}

pub fn left_sidebar_special(current_page: Page) -> Node<Msg> {
    let list_of_signed_pages: [Page; 9] = [
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
    let mut nav_links: Vec<Node<Msg>>;
    if current_page == Page::ItemSignedPage(SignedPage::Conversation) {
        nav_links = list_of_signed_pages
            .iter()
            .map(|pagex| {
                if *pagex != Page::ItemSignedPage(SignedPage::ChatWithFriends) {
                    console::log_1(&"Rust5".into());
                    console::log_1(&Page::page_name_to_string(pagex.clone()).into());
                    show_nav_link(pagex.clone())
                } else {
                    console::log_1(&Page::page_name_to_string(pagex.clone()).into());
                    show_active_nav_link(Page::page_name_to_string(pagex.clone()))
                }
            })
            .collect()
    } else {
        unimplemented!()
    }
    node! {
        <div class="sidebar d-none d-md-block">
            <h4>"Sidebar"</h4>
            <ul class="nav flex-column">
                {for el in nav_links{
                    el
                }}
            </ul>
        </div>
    }
}
