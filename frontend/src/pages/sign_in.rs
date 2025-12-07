use sauron::prelude::*;
use crate::messages::{Msg,UserLoginAttempt};
use sauron::html::{meta,title,link};
use crate::app::App;
use crate::structs::user::UserLoginData;

pub fn sign_in_display() -> Node<Msg> {
    let username = app.user_login_data.user_name.clone();
    let password = app.user_login_data.user_password.clone();
    node! {
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
<div>"Don't you have an account yet? Click" <a on_click=|_|{Msg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToRegister))}> " here " </a> "and make one."</div>
      </div>
      <div class="col-5">
      </div>
    </div>
  </div>
  </div>
  }}