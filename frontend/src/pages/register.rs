use sauron::prelude::*;
use crate::messages::{Msg,UserRegister};
use sauron::html::{meta,title,link};
use crate::list_of_pages::{Page, SharedPage, UnsignedPage,OtherPage};

pub fn register_view() -> Node<Msg> {
    log::info!("Successfully loaded Register page");

  let mut username : String = String::new();
  let mut userpassword : String = String::new();
  let mut userpassword_again : String = String::new();
  let mut usermail : String = String::new();

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
                    <button class="nav-link btn btn-link"  type="button" on_click = |_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::SignIn))}>"Sign in"</button>
                </li>
                 <li class="nav-item">
                    <a class="nav-link active btn btn-link" aria-current="page"  >"Register"</a>
                </li>
                <li class="nav-item">
                    <button class="nav-link btn btn-link" type="button" on_click = |_|{Msg::SetPage(Page::PageSortShared(SharedPage::AboutProject))}>"About project"</button>
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
        <h1 class="text-center">"Registration page"</h1>
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
        on_input=|input| Msg::Registration(UserRegister::UpdateUserRegisterName((input.value())))
    />
    <div id="passwordHelpBlock" class="form-text">
        "Enter your username."
    </div>

    <label for="inputPassword5" class="form-label">"Valid email address"</label>
    <input
        type="text"
        id="inputPassword5"
        class="form-control"
        aria-describedby="passwordHelpBlock"
        value={username.clone()}
        on_input=|input| Msg::Registration(UserRegister::UpdateUserRegisterEmail((input.value())))
    />
    <div id="passwordHelpBlock" class="form-text">
        "Enter your valid email address. A confirmation letter will be sent to that address."
    </div>

    <label for="inputPassword6" class="form-label">"Password"</label>
    <input
        type="password"
        id="inputPassword6"
        class="form-control"
        aria-describedby="passwordHelpBlock"
        value={userpassword.clone()}
        on_input=|input| Msg::Registration(UserRegister::UpdateUserRegisterPassword((input.value())))
    />
    <div id="passwordHelpBlock" class="form-text">
    "Your password must be 8-20 characters long, contain letters and numbers, and must not contain spaces, special characters, or emoji."
</div>

    <label for="inputPassword6" class="form-label">"Password checking"</label>
    <input
        type="password"
        id="inputPassword6"
        class="form-control"
        aria-describedby="passwordHelpBlock"
        value={userpassword_again.clone()}
        on_input=|input| Msg::Registration(UserRegister::UpdateUserRegisterPasswordAgain((input.value())))
    />
    <div id="passwordHelpBlock" class="form-text">
        "Enter chosen password again. This step is necessary in order to prevent typos."
    </div>


    <button class="btn btn-primary w-100" on_click=move |_| Msg::Registration(UserRegister::RegistrationAttempt)>
        "Register"
    </button>
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