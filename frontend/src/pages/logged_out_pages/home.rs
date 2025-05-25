use crate::messages::{GoToPage, DefinedMsg, SwitchToPageSigned, SwitchToPageUnsigned,SwitchToPageShared};
use sauron::prelude::*;

pub fn view() -> Node<DefinedMsg> {
    node! {
          <main>
        <nav class="navbar navbar-expand navbar-dark bg-dark">
        <div class="container-fluid">
            <a class="navbar-brand" href="#">"MathNet"</a>
            <div class="collapse navbar-collapse">
                <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                <li class="nav-item">
                <li class="nav-item">
                        <a class="nav-link active" aria-current="page" href="#">"Home page"</a>
                    </li>
            </li>
            <li class="nav-item">
                        <a class="nav-link" on_click=|_| {DefinedMsg::SetPage(GoToPage::GoToPageShared(SwitchToPageShared::GoToDocsPage))}>"Docs"</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" on_click=|_| {DefinedMsg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToLoginPage))}>"Log in"</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" on_click=|_| {DefinedMsg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToRegister))}>"Register"</a>
                    </li>
                    <li class="nav-item">
                    <a class="nav-link" on_click=|_| {DefinedMsg::SetPage(GoToPage::GoToPageShared(SwitchToPageShared::GoToAboutProject))}>"About this project"</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link" on_click=|_| {DefinedMsg::SetPage(GoToPage::GoToPageShared(SwitchToPageShared::GoToPrivacyAndSecurity))}>"Privacy and security"</a>
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
          <h1 class="text-center">"Welcome to MathNet!!!"</h1>
          <p class="basicparagraph" class="text-start">"Welcome to MathNet. Hope you will enjoy it!"</p>
        </div>
        <div class="col-2">
        </div>
      </div>
    </div>
      </main>
      }
}
