use crate::messages::{GoToPage, Msg, SwitchToPageSigned, SwitchToPageUnsigned};
use sauron::prelude::*;
//use sauron::html::{meta,title,link};

pub fn view() -> Node<Msg> {
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
            <a class="nav-link active" aria-current="page" href="#">"Docs"</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" on_click=|_| {Msg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToLoginPage))}>"Log in"</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" on_click=|_| {Msg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToRegister))}>"Register"</a>
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
          <h1 class="text-center">"Docs"</h1>
        </div>
        <p class="basicparagraph" class="text-start">"Code for this project is available "<a href="https://github.com/iruspro/mathnet.git">"here"</a>"."</p>
        <div class="col-2">
        </div>
      </div>
    </div>

    </main>
    }
}
