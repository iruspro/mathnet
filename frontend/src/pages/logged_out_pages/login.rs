use sauron::prelude::*;
use crate::messages::{Msg,GoToPage,UserLoginAttempt};
use sauron::html::{meta,title,link};
use crate::app::App;
use crate::user::UserLoginData;

pub fn view(app : &App) -> Node<Msg> {
    let username = app.user_login_data.user_name.clone();
    let password = app.user_login_data.user_password.clone();
    node! {
      <main>
    <nav class="navbar navbar-expand navbar-dark bg-dark">
    <div class="container-fluid">
        <a class="navbar-brand" href="#">"MathNet"</a>
        <div class="collapse navbar-collapse">
            <ul class="navbar-nav me-auto mb-2 mb-lg-0">
            <li class="nav-item">
            <a class="nav-link" on_click=|_|{Msg::SetPage(GoToPage::GoToHomePage)}>"Home page"</a>
        </li>
        <li class="nav-item">
                    <a class="nav-link" on_click=|_| {Msg::SetPage(GoToPage::GoToDocsPage)}>"Docs"</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link active" aria-current="page" href="#">"Login"</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" on_click=|_| {Msg::SetPage(GoToPage::GoToRegister)}>"Register"</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" on_click=|_| {Msg::SetPage(GoToPage::GoToAboutProject)}>"About this project"</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" on_click=|_| {Msg::SetPage(GoToPage::GoToPrivacyAndSecurity)}>"Privacy and security"</a>
                </li>
            </ul>
        </div>
    </div>
    </nav>
    <p>"Login"</p>
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
    </button>

    </main>
  }}