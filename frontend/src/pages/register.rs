use sauron::prelude::*;
use crate::messages::{Msg,UserRegister};
use sauron::html::{meta,title,link};

pub fn register_display() -> Node<Msg> {
  let mut username : String = String::new();
  let mut userpassword : String = String::new();
  let mut userpassword_again : String = String::new();
  let mut usermail : String = String::new();

    node! { 
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
  }}