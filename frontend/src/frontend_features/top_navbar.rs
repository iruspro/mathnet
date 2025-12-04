
/*
Enabling top navbar on unsigned pages. This file also containts a system for 
top navbar link management.
*/


use sauron::prelude::*;
use crate::messages::GoToPage::GoToPageSigned;
use crate::messages::{Msg};
use crate::app::App;
use crate::logics::{displaying_friends::{show_friends_at_sidebar,show_chats_in_content}, displaying_conversation};
use crate::experimental_examples::imaginary_friends;
use crate::list_of_pages::{Page,SignedPage,OtherPage, list_of_unsigned_pages};
use sauron::node;
use crate::web_sys::console;

fn show_nav_buttons(page : &Page)->Node<Msg>{
match page{
    Page::PageSortUnsigned(UnsignedPage::Home) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" type="button" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::Home))}>"Home"</button>
                </li>
        }
    },
    Page::PageSortUnsigned(UnsignedPage::LogIn) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" type="button" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::LogIn))}>"Log in"</button>
                </li>
        }
    },
    Page::PageSortUnsigned(UnsignedPage::Register) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" type="button" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::Register))}>"Register"</button>
                </li>
        }
    },
    Page::PageSortUnsigned(UnsignedPage::Docs) =>{
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" type="button" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::Docs))}>"Docs"</button>
                </li>
        }
    } ,
    Page::PageSortUnsigned(UnsignedPage::AboutProject) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" type="button" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::AboutProject))}>"About project"</button>
                </li>
        }
    },
    Page::PageSortUnsigned(UnsignedPage::PrivacyAndSecurity) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" type="button" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::PrivacyAndSecurity))}>"Privacy and Security"</button>
                </li>
        }
    },
}
}


fn show_active_nav_button(page: &Page)->Node<Msg>{
   match page{
    Page::PageSortUnsigned(UnsignedPage::Home) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" class="myactivenavbutton" type="button" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::Home))}>"Home"</button>
                </li>
        }
    },
    Page::PageSortUnsigned(UnsignedPage::LogIn) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" class="myactivenavbutton" type="button" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::LogIn))}>"Log in"</button>
                </li>
        }
    },
    Page::PageSortUnsigned(UnsignedPage::Register) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" class="myactivenavbutton" type="button" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::Register))}>"Register"</button>
                </li>
        }
    },
    Page::PageSortUnsigned(UnsignedPage::Docs) =>{
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" type="button" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::Docs))}>"Docs"</button>
                </li>
        }
    } ,
    Page::PageSortUnsigned(UnsignedPage::AboutProject) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" class="myactivenavbutton" type="button" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::AboutProject))}>"About project"</button>
                </li>
        }
    },
    Page::PageSortUnsigned(UnsignedPage::PrivacyAndSecurity) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link"  class="myactivenavbutton" type="button" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::PrivacyAndSecurity))}>"Privacy and Security"</button>
                </li>
        }
    },
}
}

pub fn display_top_navbar(current_page: &Page)-> Node<MSG>{
    /* 
    Generate nav links as a Vec<Node<Msg>> properly such that 
    the current page's link is the active one.
    */
    let nav_buttons: Vec<Node<Msg>> = list_of_unsigned_pages
        .iter()
        .map(|pagex| {
            if *pagex == current_page {
                show_active_nav_button(pagex)
            } else {
                show_nav_button(pagex)
            }
        })
        .collect();

    node!{<nav class="navbar bg-dark navbar-expand-lg bg-body-tertiary" data-bs-theme="dark">
            <div class="container-fluid">
            <a class="navbar-brand" href="#">"Navbar"</a>
    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarTogglerDemo02" aria-controls="navbarTogglerDemo02" aria-expanded="false" aria-label="Toggle navigation">
      <span class="navbar-toggler-icon"></span>
    </button>
    <div class="collapse navbar-collapse" id="navbarTogglerDemo02">
              <ul class="navbar-nav me-auto mb-2 mb-lg-0 navbar-nav">
               {for el in nav_buttons:{
                el
               }}
              </ul>
            </div></div>
          </nav>}
}