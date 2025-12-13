/*
Enabling left sidebar on signed pages. This file also containts a system for 
left sidebar link management.
*/

/*
use sauron::prelude::*;
use crate::messages::{Msg};
use crate::app::App;
use crate::experimental_examples::imaginary_friends;
use crate::list_of_pages::{Page,SignedPage,OtherPage, SharedPage, list_of_signed_pages, list_of_unsigned_pages};
use sauron::node;
use crate::web_sys::console;

fn show_nav_link(page : &Page)->Node<Msg>{
match page{
    Page::PageSortShared(SharedPage::AboutProject) => {
        node!{
             <li >
                  <a class="mylink" on_click=|_|{Msg::SetPage(Page::PageSortShared(SharedPage::AboutProject))}>"About project"</a>
                </li>
        }
    },
    Page::PageSortShared(SharedPage::Docs) => {
        node!{
             <li >
                  <a class="mylink" on_click=|_|{Msg::SetPage(Page::PageSortShared(SharedPage::Docs))}>"Docs"</a>
                </li>
        }
    },
    Page::PageSortShared(SharedPage::PrivacyAndSecurity) => {
        node!{
             <li >
                  <a class="mylink" on_click=|_|{Msg::SetPage(Page::PageSortShared(SharedPage::PrivacyAndSecurity))}>"Privacy and Security"</a>
                </li>
        }
    },
    Page::PageSortSigned(SignedPage::Settings) => {
        node!{
             <li >
                  <a class="mylink" on_click=|_|{Msg::SetPage(Page::PageSortSigned(SignedPage::Settings))}>"Settings"</a>
                </li>
        }},
    Page::PageSortSigned(SignedPage::MathNetNotifications) => {
        node!{
             <li >
                  <a class="mylink" on_click=|_|{Msg::SetPage(Page::PageSortSigned(SignedPage::MathNetNotifications))}>"MathNet Notifications"</a>
                </li>
        }
    },
    Page::PageSortSigned(SignedPage::UserProfile) => {
        node!{
             <li >
                  <a class="mylink" on_click=|_|{Msg::SetPage(Page::PageSortSigned(SignedPage::UserProfile))}>"User profile"</a>
                </li>
        }
    },
    Page::PageSortSigned(SignedPage::GroupList) => {
        node!{
             <li >
                  <a class="mylink" on_click=|_|{Msg::SetPage(Page::PageSortSigned(SignedPage::GroupList))}>"Group lists"</a>
                </li>
        }
    },
    Page::PageSortSigned(SignedPage::ChatWithFriends) => {
        node!{
             <li >
                  <a class="mylink" on_click=|_|{Msg::SetPage(Page::PageSortSigned(SignedPage::ChatWithFriends))}>"Chat with Friends"</a>
                </li>
        }
    },
    _ => unimplemented!("Unimplemented show_nav_link for this page.")
}
}


fn show_active_nav_link(page : &Page)->Node<Msg>{
   match page{
    Page::PageSortShared(SharedPage::AboutProject) => {
        node!{
             <li >
                  <a class="mycurrentlyactivelink" >"About project"</a>
                </li>
        }
    },
    Page::PageSortShared(SharedPage::Docs) => {
        node!{
             <li >
                  <a class="mycurrentlyactivelink" >"Docs"</a>
                </li>
        }
    },
    Page::PageSortShared(SharedPage::PrivacyAndSecurity) => {
        node!{
             <li >
                  <a class="mycurrentlyactivelink" >"Privacy and Security"</a>
                </li>
        }
    },
    Page::PageSortSigned(SignedPage::Settings) => {
        node!{
             <li >
                  <a class="mycurrentlyactivelink" >"Settings"</a>
                </li>
        }},
    Page::PageSortSigned(SignedPage::MathNetNotifications) => {
        node!{
             <li >
                  <a class="mycurrentlyactivelink" >"MathNet Notifications"</a>
                </li>
        }
    },
    Page::PageSortSigned(SignedPage::UserProfile) => {
        node!{
             <li >
                  <a class="mycurrentlyactivelink" >"User profile"</a>
                </li>
        }
    },
    Page::PageSortSigned(SignedPage::GroupList) => {
        node!{
             <li >
                  <a class="mycurrentlyactivelink" >"Group lists"</a>
                </li>
        }
    },
    Page::PageSortSigned(SignedPage::ChatWithFriends) => {
        node!{
             <li >
                  <a class="mycurrentlyactivelink">"Chat with Friends"</a>
                </li>
        }
    },
    _ => unimplemented!("Unimplemented show_nav_link for this page.")
}

}



pub fn display_left_sidebar(current_page: &Page) -> Node<Msg> {
    /* 
    Generate nav links as a Vec<Node<Msg>> properly such that 
    the current page's link is the active one.
    */
    log::info!("Rendering left sidebar for current page.");
    node! {
        <ul class="mysidebar">
        {for el in list_of_signed_pages.iter(){
            if *el == *current_page {
                    show_active_nav_link(el)
                } else {
                    show_nav_link(el)
                } 
        }}
        </ul>
        <div class="my-stacked-sidebar">
                <nav class="navbar bg-dark bg-body-tertiary" data-bs-theme="dark">
                    <div class="container-fluid">
                    <a class="navbar-brand" href="#">"Navbar"</a>
            <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarTogglerDemo02" aria-controls="navbarTogglerDemo02" aria-expanded="false" aria-label="Toggle navigation">
              <span class="navbar-toggler-icon"></span>
            </button>
            <div class="collapse navbar-collapse" id="navbarTogglerDemo02">
                      <ul class="navbar-nav me-auto mb-2 mb-lg-0 navbar-nav">
                        {for el in list_of_signed_pages.iter(){
                          if *el == *current_page {
                    show_active_nav_link(el)
                } else {
                    show_nav_link(el)
                } 
                        }}
                      </ul>
                    </div></div>
                  </nav>
                  </div>
    }
}
*/

use sauron::prelude::*;
use crate::messages::Msg;
use crate::list_of_pages::{Page, SignedPage, UnsignedPage, SharedPage};

type MSG = Msg;

fn sidebar_button_active(label: &str) -> Node<Msg> {
    node!{
        <div class="list-group-item active" aria-current="true">
            { text(label) }
        </div>
    }
}

fn sidebar_button_inactive(label: &str, target: Page) -> Node<Msg> {
    node!{
        <button class="list-group-item list-group-item-action" type="button" on_click=move |_| Msg::SetPage(target.clone())>
            { text(label) }
        </button>
    }
}

/// Render one sidebar item: active if `current_page == target`, otherwise clickable.
/// Note: clone() on Page requires Page: Clone — adjust if your Page type differs.
fn show_sidebar_item(current_page: &Page, target: Page, label: &str) -> Node<Msg> {
    
        sidebar_button_active(label)
}

/// Main left sidebar. Build items inline so Sauron unrolls node lists immediately.
/// Replace/add items to match your application's pages.
pub fn display_left_sidebar(current_page: &Page) -> Node<MSG> {
    log::info!("Loading left sidebar.");
    node!{
        <aside class="left-sidebar">
            <div class="list-group">
                show_sidebar_item(current_page, Page::PageSortSigned(SignedPage::Conversation), "Conversations") 
                show_sidebar_item(current_page, Page::PageSortSigned(SignedPage::UserProfile), "Profile") 
                <hr/>
                show_sidebar_item(current_page, Page::PageSortUnsigned(UnsignedPage::Home), "Home")
                show_sidebar_item(current_page, Page::PageSortShared(SharedPage::Docs), "Docs")
            </div>
        </aside>
    }
}