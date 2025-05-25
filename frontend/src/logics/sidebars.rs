use std::thread::current;

use crate::app::App;
use crate::experimental_examples::imaginary_friends;
use crate::list_of_pages::{OtherPage, Page, SignedPage,SharedPage,page_name_to_string};
use crate::logics::{
    displaying_conversation,
    displaying_friends::{show_chats_in_content, show_friends_at_sidebar},
};
use crate::logics::converting_ids_to_string::string_to_page;
use crate::messages::{DefinedMsg, GoToPage,SwitchToPageOther,SwitchToPageSigned,SwitchToPageUnsigned,SwitchToPageShared};
use crate::pages::logged_in_pages::{exercise, list_of_exercises};
use crate::web_sys::console;
use sauron::node;
use sauron::prelude::*;


fn show_nav_link(page: Page) -> Node<DefinedMsg> {
    match page {
        Page::ItemSignedPage(SignedPage::GroupsList) => node! {
            <li class="nav-item">
                <a class="nav-link" on_click=|_| {
                    DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroupsList))
                }>"Groups"</a>
            </li>
        },
        Page::ItemSignedPage(SignedPage::UserProfile(user_id)) => node! {
            <li class="nav-item">
                <a class="nav-link" on_click=move|_| {
                    DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile(user_id.clone())))
                }>"User profile"</a>
            </li>
        },
        Page::ItemSignedPage(SignedPage::Settings) => node! {
            <li class="nav-item">
                <a class="nav-link" on_click=|_| {
                    DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToSettings))
                }>"Settings"</a>
            </li>
        },
        Page::ItemSignedPage(SignedPage::Notifications) => node! {
            <li class="nav-item">
                <a class="nav-link" on_click=|_| {
                    DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToNotifications))
                }>"Notifications"</a>
            </li>
        },
        Page::ItemSignedPage(SignedPage::ChatWithFriends) => node! {
            <li class="nav-item">
                <a class="nav-link" on_click=|_| {
                    DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToChatWithFriends))
                }>"Chat with friends"</a>
            </li>
        },
        /* Page::ItemSignedPage(SignedPage::Chat(_)) => node! {
            <li class="nav-item">
                <a class="nav-link">"Chat (not linked)"</a>
            </li>
        },*/
        Page::ItemSignedPage(SignedPage::Friends) => node! {
            <li class="nav-item">
                <a class="nav-link" on_click=|_| {
                    DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToFriends))
                }>"Friends"</a>
            </li>
        },
        Page::ItemSignedPage(SignedPage::Profile(user_id)) => node! {
            <li class="nav-item">
                <a class="nav-link" on_click=move|_| {
                    DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToProfile(user_id.clone())))
                }>"Profile"</a>
            </li>
        },
        /*
        Page::ItemSignedPage(SignedPage::Exercise(list_of_exercises_id, exercise_id)) => node! {
            <li class="nav-item">
                <a class="nav-link" on_click=move|_| {
                    DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToExercise(list_of_exercises_id.clone(),exercise_id.clone())))
                }>"Exercise"</a>
            </li>
        },
        
        Page::ItemSignedPage(SignedPage::ListOfExercises(list_of_exercises_id)) => node! {
            <li class="nav-item">
                <a class="nav-link" on_click=move|_| {
                    DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToListOfExercises(list_of_exercises_id.clone())))
                }>"List of exercises"</a>
            </li>
        },
        Page::ItemSignedPage(SignedPage::Group(group_id)) => node! {
            <li class="nav-item">
                <a class="nav-link" on_click=move|_| {
                    DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroup(group_id.clone())))
                }>"Group"</a>
            </li>
        },
        */
        Page::ItemSignedPage(SignedPage::UserSuggestsToDevelopers) => node! {
            <li class="nav-item">
                <a class="nav-link" on_click=|_| {
                    DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserSuggestsDevelopers))
                }>"Suggest to the developers"</a>
            </li>
        },
        _ => node! {
            <li class="nav-item">
                <a class="nav-link">"Unknown page"</a>
            </li>
        },
    }
}

/*Old show_nav_link function

fn show_nav_link(page: Page) -> Node<DefinedMsg> {
    match page {
        Page::ItemSignedPage(SignedPage::UserProfile(_)) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile))}>"User profile"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::ChatWithFriends) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToChatWithFriends))}>"Chat with friends"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::GroupsList) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroupsList))}>"Groups"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::Docs) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToDocsPage))}>"Docs"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::AboutProject) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToAboutProject))}>"About this project"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::PrivacyAndSecurity) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToPrivacyAndSecurity))}>"Privacy and security"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::Notifications) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToNotifications))}>"Notifications"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::Settings) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToSettings))}>"Settings"</a>
                            </li>
            }
        }
        Page::ItemSignedPage(SignedPage::LogOut) => {
            node! {<li class="nav-item">
                                <a class="nav-link" on_click=|_|{DefinedMsg::SetPage(GoToPage::GoToPageOther(SwitchToPageOther::GoToLogOut))}>"Log out"</a>
                            </li>
            }
        }
        _ => unimplemented!("Dummy mistake at show_nav_link"),
    }
}
    */

fn show_active_nav_link(page_name: String) -> Node<DefinedMsg> {
    if page_name == "Groups list" {
        node! {
        <li class="nav-item">
                        <a class="nav-link text-white" arria-current="page" href="#">{text("Groups".to_string())}</a>
                    </li>
            }
    } else {
        if page_name == "Chat with friends" {
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


//pub fn left_sidebar(current_page: Page) -> Node<DefinedMsg> {
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
//    let nav_links: Vec<Node<DefinedMsg>> = list_of_signed_pages
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
pub fn left_sidebar(current_page : Page) -> Node<DefinedMsg>{
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
//pub fn right_sidebar() -> Node<DefinedMsg> {
//node!{
//    <div class="right-sidebar d-none d-md-block text-white">
//                <h4>"Right Sidebar"</h4>
//                <ul class="nav flex-column">
//                <div class="search-container">
//        <input type="text" class="form-control search-input" placeholder="Find a person" on_input=|input|{DefinedMsg::SearchFriend(input.value())}></input>
//        <i class="fas fa-search search-icon"></i>
//      </div>
//                    //{show_friends_at_sidebar()};
//                </ul>
//            </div>
//
//}}}

use sauron::prelude::*;

// Assume DefinedMsg and Page/SignedPage types are already defined

pub fn left_sidebar(current_page: Page) -> Node<DefinedMsg> {
    let page_titles: [String; 9] = [
    "Groups list".to_string(),
    "User profile".to_string(),
    "Settings".to_string(),
    "Notifications".to_string(),
    "Chat with friends".to_string(),
    //"Chat".to_string(),
    "Friends".to_string(),
    "Profile".to_string(),
    //"Exercise".to_string(),
    "List_of_exercises".to_string(),
    //"Group".to_string(),
    "Suggest to the developers".to_string(),
];

let string_of_current_page = page_name_to_string(current_page.clone());
    // Generate nav links as a Vec<Node<DefinedMsg>>
    let nav_links: Vec<Node<DefinedMsg>> = page_titles
        .iter()
        .map(|pagex| {
            if *pagex != string_of_current_page {
                show_nav_link(current_page.clone())
            } else {
                show_active_nav_link(pagex.clone())
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

pub fn left_sidebar_special(current_page: Page) -> Node<DefinedMsg> {
 let page_titles: [String; 9] = [
    "Groups list".to_string(),
    "User profile".to_string(),
    "Settings".to_string(),
    "Notifications".to_string(),
    "Chat with friends".to_string(),
    //"Chat".to_string(),
    "Friends".to_string(),
    "Profile".to_string(),
    //"Exercise".to_string(),
    "List_of_exercises".to_string(),
    //"Group".to_string(),
    "Suggest to the developers".to_string(),
];
    let string_of_current_page = page_name_to_string(current_page.clone());
    let mut nav_links: Vec<Node<DefinedMsg>>;
    if string_of_current_page == "Chat".to_string() {
        nav_links = page_titles
            .iter()
            .map(|pagex| {
                if *pagex != "Chat with friends".to_string() {
                    show_nav_link(string_to_page(pagex.clone()))
                } else {
                    show_active_nav_link(pagex.clone())
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
