use sauron::prelude::*;
use crate::list_of_pages::{UnsignedPage, SharedPage};
use crate::messages::{Msg,UserLoginAttempt};
use sauron::html::{meta,title,link};
use crate::app::App;
use crate::structs::user::UserLoginData;
use crate::list_of_pages::Page;

pub fn sign_in_view(app: &App) -> Node<Msg> {
    log::info!("Successfully loaded sign in page");
    
    let username = app.user_sign_in_data.user_name.clone();
    let password = app.user_sign_in_data.user_password.clone();
    node! {
<nav class="navbar bg-dark navbar-expand-lg bg-body-tertiary" data-bs-theme="dark">
            <div class="container-fluid">
                <a class="navbar-brand" href="#">"Navbar"</a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarTogglerDemo02" aria-controls="navbarTogglerDemo02" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarTogglerDemo02">
                    <ul class="navbar-nav me-auto mb-2 mb-lg-0 navbar-nav">
                <li class="nav-item">
                    <button class="nav-link btn btn-link"  type="button" on_click = |_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::Home))}>"Home"</button>
                </li>
<li class="nav-item">
                    <a class="nav-link active btn btn-link" aria-current="page"  >"Sign in"</a>
                </li>
                <li class="nav-item">
                    <button class="nav-link btn btn-link"  type="button" on_click = |_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::Register))}>"Register"</button>
                </li>
                 <li class="nav-item">
                    <button class="nav-link btn btn-link"  type="button" on_click = |_|{Msg::SetPage(Page::PageSortShared(SharedPage::AboutProject))}>"About project"</button>
                </li>
                <li class="nav-item">
                    <button class="nav-link btn btn-link" type="button" on_click = |_|{Msg::SetPage(Page::PageSortShared(SharedPage::Docs))}>"Docs"</button>
                </li>
                    </ul>
                </div>
            </div>
        </nav>


    <div class="container-fluid">
    <div class="row">
      <div class="col-2">
        
      </div>
      <div class="col-8" class="col text-start">
        <h1 class="text-center">"Login"</h1>
        <div class="col-2">
      </div>
      </div>
      <div class="row">
      <div class="col-5">
      </div>
      <div class="col-2">
        <label for="inputPassword5" class="form-label">"Username"</label>
    <input
        type="text"
        id="inputPassword5"
        class="form-control"
        aria-describedby="passwordHelpBlock"
        value={username.clone()}
        on_input=|input| Msg::LoginAttempt(UserLoginAttempt::UpdateUserName((input.value())))
    />
    <div id="passwordHelpBlock" class="form-text">
        "Enter your username."
    </div>

    <label for="inputPassword6" class="form-label">"Password"</label>
    <input
        type="password"
        id="inputPassword6"
        class="form-control"
        aria-describedby="passwordHelpBlock"
        value={password.clone()}
        on_input=|input| Msg::LoginAttempt(UserLoginAttempt::UpdateUserPassword((input.value())))
    />
    <div id="passwordHelpBlock" class="form-text">
        "Your password must be 8-20 characters long, contain letters and numbers, and must not contain spaces, special characters, or emoji."
    </div>
    <button class="btn btn-primary w-100" on_click=move |_| Msg::LoginAttempt(UserLoginAttempt::CheckLoginValidy)>
        "Log in"
    </button> <br></br>
<div>"Don't you have an account yet? Click" <a on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::Register))}> " here " </a> "and make one."</div>
      </div>
      <div class="col-5">
      </div>
    </div>
  </div>
  </div>
  <div class="myfooter">
        "Notification:"
        <a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-nc-nd/4.0/88x31.png" /></a><br />"This work is licensed under a" <a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/">"Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International License"</a>.
        </div>

  }}