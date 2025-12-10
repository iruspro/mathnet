
/*
Enabling top navbar on unsigned pages. This file also containts a system for 
top navbar link management.
*/

/*
use sauron::prelude::*;
use crate::messages::{Msg};
use crate::app::App;
use crate::experimental_examples::imaginary_friends;
use crate::list_of_pages::{Page,SignedPage,UnsignedPage,OtherPage, SharedPage, list_of_unsigned_pages};
use sauron::node;
use crate::web_sys::console;

type MSG = Msg;

fn show_nav_button(page : &Page) -> Node<Msg>{
match page{
    Page::PageSortUnsigned(UnsignedPage::Home) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" type="button" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::Home))}>"Home"</button>
                </li>
        }
    },
    Page::PageSortUnsigned(UnsignedPage::SignIn) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" type="button" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::SignIn))}>"Sign in"</button>
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
    Page::PageSortShared(SharedPage::Docs) =>{
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" type="button" on_click=|_|{Msg::SetPage(Page::PageSortShared(SharedPage::Docs))}>"Docs"</button>
                </li>
        }
    } ,
    Page::PageSortShared(SharedPage::AboutProject) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" type="button" on_click=|_|{Msg::SetPage(Page::PageSortShared(SharedPage::AboutProject))}>"About project"</button>
                </li>
        }
    },
    Page::PageSortShared(SharedPage::PrivacyAndSecurity) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" type="button" on_click=|_|{Msg::SetPage(Page::PageSortShared(SharedPage::PrivacyAndSecurity))}>"Privacy and Security"</button>
                </li>
        }
    },
    _ => panic!("Should match this unsigned page")
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
    Page::PageSortUnsigned(UnsignedPage::SignIn) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" class="myactivenavbutton" type="button" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::SignIn))}>"Log in"</button>
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
    Page::PageSortShared(SharedPage::Docs) =>{
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" type="button" on_click=|_|{Msg::SetPage(Page::PageSortShared(SharedPage::Docs))}>"Docs"</button>
                </li>
        }
    } ,
    Page::PageSortShared(SharedPage::AboutProject) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link" class="myactivenavbutton" type="button" on_click=|_|{Msg::SetPage(Page::PageSortShared(SharedPage::AboutProject))}>"About project"</button>
                </li>
        }
    },
    Page::PageSortShared(SharedPage::PrivacyAndSecurity) => {
        node!{
            <li class="nav-item">
                  <button class="nav-link btn btn-link"  class="myactivenavbutton" type="button" on_click=|_|{Msg::SetPage(Page::PageSortShared(SharedPage::PrivacyAndSecurity))}>"Privacy and Security"</button>
                </li>
        }
    },
    _ => panic!("Shouldn't match this arm.")
}
}

pub fn display_top_navbar(current_page: &Page)-> Node<MSG>{
    /* 
    Generate nav links as a Vec<Node<Msg>> properly such that 
    the current page's link is the active one.
    */
    log::info!("Loading top navbar for unsigned pages.");
    node!{<nav class="navbar bg-dark navbar-expand-lg bg-body-tertiary" data-bs-theme="dark">
            <div class="container-fluid">
            <a class="navbar-brand" href="#">"Navbar"</a>
    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarTogglerDemo02" aria-controls="navbarTogglerDemo02" aria-expanded="false" aria-label="Toggle navigation">
      <span class="navbar-toggler-icon"></span>
    </button>
    <div class="collapse navbar-collapse" id="navbarTogglerDemo02">
              <ul class="navbar-nav me-auto mb-2 mb-lg-0 navbar-nav">
               {for el in list_of_unsigned_pages.iter(){
                if *el == *current_page {
                    show_active_nav_button(el)
                } else {
                    show_nav_button(el)
                } 
               }}
              </ul>
            </div></div>
          </nav>}
}
          */

     // ...existing code...
use sauron::prelude::*;
use crate::messages::Msg;
use crate::list_of_pages::{Page, SignedPage, UnsignedPage, SharedPage, list_of_unsigned_pages};

type MSG = Msg;

fn show_nav_button(page: &Page) -> Node<Msg> {
    match page {
        Page::PageSortUnsigned(UnsignedPage::Home) => {
            node!{
                <li class="nav-item">
                    <button class="nav-link btn btn-link" type="button" on_click=|_| Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::Home))>"Home"</button>
                </li>
            }
        }
        Page::PageSortUnsigned(UnsignedPage::SignIn) => {
            node!{
                <li class="nav-item">
                    <button class="nav-link btn btn-link" type="button" on_click=|_| Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::SignIn))>"Sign in"</button>
                </li>
            }
        }
        Page::PageSortUnsigned(UnsignedPage::Register) => {
            node!{
                <li class="nav-item">
                    <button class="nav-link btn btn-link" type="button" on_click=|_| Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::Register))>"Register"</button>
                </li>
            }
        }
        Page::PageSortShared(SharedPage::Docs) => {
            node!{
                <li class="nav-item">
                    <button class="nav-link btn btn-link" type="button" on_click=|_| Msg::SetPage(Page::PageSortShared(SharedPage::Docs))>"Docs"</button>
                </li>
            }
        }
        Page::PageSortShared(SharedPage::AboutProject) => {
            node!{
                <li class="nav-item">
                    <button class="nav-link btn btn-link" type="button" on_click=|_| Msg::SetPage(Page::PageSortShared(SharedPage::AboutProject))>"About"</button>
                </li>
            }
        }
        Page::PageSortShared(SharedPage::PrivacyAndSecurity) => {
            node!{
                <li class="nav-item">
                    <button class="nav-link btn btn-link" type="button" on_click=|_| Msg::SetPage(Page::PageSortShared(SharedPage::PrivacyAndSecurity))>"Privacy"</button>
                </li>
            }
        }
        _ => {
            // fallback: render nothing but keep node well-formed
            node!{ <li class="nav-item"></li> }
        }
    }
}

fn show_active_nav_button(page: &Page) -> Node<Msg> {
    match page {
        Page::PageSortUnsigned(UnsignedPage::Home) => {
            node!{
                <li class="nav-item">
                    <button class="nav-link active btn btn-link" aria-current="page" type="button">"Home"</button>
                </li>
            }
        }
        Page::PageSortUnsigned(UnsignedPage::SignIn) => {
            node!{
                <li class="nav-item">
                    <button class="nav-link active btn btn-link" aria-current="page" type="button">"Sign in"</button>
                </li>
            }
        }
        Page::PageSortUnsigned(UnsignedPage::Register) => {
            node!{
                <li class="nav-item">
                    <button class="nav-link active btn btn-link" aria-current="page" type="button">"Register"</button>
                </li>
            }
        }
        Page::PageSortShared(SharedPage::Docs) => {
            node!{
                <li class="nav-item">
                    <button class="nav-link active btn btn-link" aria-current="page" type="button">"Docs"</button>
                </li>
            }
        }
        Page::PageSortShared(SharedPage::AboutProject) => {
            node!{
                <li class="nav-item">
                    <button class="nav-link active btn btn-link" aria-current="page" type="button">"About"</button>
                </li>
            }
        }
        Page::PageSortShared(SharedPage::PrivacyAndSecurity) => {
            node!{
                <li class="nav-item">
                    <button class="nav-link active btn btn-link" aria-current="page" type="button">"Privacy"</button>
                </li>
            }
        }
        _ => node!{ <li class="nav-item"></li> },
    }
}

pub fn display_top_navbar(current_page: &Page) -> Node<MSG> {
    log::info!("Loading top navbar for unsigned pages.");
    node!{
        <nav class="navbar bg-dark navbar-expand-lg bg-body-tertiary" data-bs-theme="dark">
            <div class="container-fluid">
                <a class="navbar-brand" href="#">"Navbar"</a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarTogglerDemo02" aria-controls="navbarTogglerDemo02" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarTogglerDemo02">
                    <ul class="navbar-nav me-auto mb-2 mb-lg-0 navbar-nav">
                        { for el in list_of_unsigned_pages.iter() {
                                show_nav_button(el)
                        }}
                    </ul>
                </div>
            </div>
        </nav>
    }
}     