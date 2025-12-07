/*
Enabling left sidebar on signed pages. This file also containts a system for 
left sidebar link management.
*/


use sauron::prelude::*;
use crate::messages::{Msg};
use crate::app::App;
use crate::logics::{displaying_friends::{show_friends_at_sidebar,show_chats_in_content}};
use crate::experimental_examples::imaginary_friends;
use crate::list_of_pages::{Page,SignedPage,OtherPage, SharedPage, list_of_signed_pages};
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
    let nav_links: Vec<Node<Msg>> = list_of_signed_pages
        .iter()
        .map(|pagex| {
            if *pagex == *current_page {
                show_active_nav_link(pagex)
            } else {
                show_nav_link(pagex)
            }
        })
        .collect();

    node! {
        <ul class="mysidebar">
        {for el in nav_links.clone(){
            el
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
                        {for el in nav_links{
                          el
                        }}
                      </ul>
                    </div></div>
                  </nav>
                  </div>
    }
}
