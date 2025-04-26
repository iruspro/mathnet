use sauron::prelude::*;
use crate::messages::{Msg,GoToPage};
use sauron::html::{meta,title,link};

pub fn view() -> Node<Msg> {
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
                    <a class="nav-link" on_click=|_| {Msg::SetPage(GoToPage::GoToLogin)}>"Login"</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link active" aria-current="page" href="#">"Register"</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" on_click=|_| {Msg::SetPage(GoToPage::GoToAboutProject)}>"About this project"</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" on_click=|_| {Msg::SetPage(GoToPage::GoToPrivacyAndSecurity
                    )}>"Privacy and security"</a>
                </li>
            </ul>
        </div>
    </div>
    </nav>
    <div class="container-fluid">
  <div class="row">
    <div class="col-2">
      1 of 3
    </div>
    <div class="col-2">
      2 of 3 (wider)
    </div>
    <div class="col-2">
      3 of 3
    </div>
  </div>
</div>
  </main>
  }}