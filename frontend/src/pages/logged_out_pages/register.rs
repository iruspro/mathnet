use sauron::prelude::*;
use crate::messages::{Msg,GoToPage,UserRegister,SwitchToPageSigned,SwitchToPageUnsigned};
use sauron::html::{meta,title,link};

pub fn view() -> Node<Msg> {
  let mut username : String = String::new();
  let mut userpassword : String = String::new();
  let mut userpassword_again : String = String::new();
  let mut usermail : String = String::new();

    node! {
      <main>
    <nav class="navbar navbar-expand navbar-dark bg-dark">
    <div class="container-fluid">
        <a class="navbar-brand" href="#">"MathNet"</a>
        <div class="collapse navbar-collapse">
            <ul class="navbar-nav me-auto mb-2 mb-lg-0">
            <li class="nav-item">
            <a class="nav-link" on_click=|_|{Msg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToHomePage))}>"Home page"</a>
        </li>
        <li class="nav-item">
                    <a class="nav-link" on_click=|_| {Msg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToDocsPage))}>"Docs"</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" on_click=|_| {Msg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToLoginPage))}>"Log in"</a>
                </li>
                <li class="nav-item">
                <a class="nav-link active" aria-current="page" href="#">"Register"</a>
                </li>
                <li class="nav-item">
                <a class="nav-link" on_click=|_| {Msg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToAboutProject))}>"About this project"</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" on_click=|_| {Msg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToPrivacyAndSecurity))}>"Privacy and security"</a>
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
        "Enter your valid email address."
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
  </main>
  }}