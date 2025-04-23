use sauron::prelude::*;
use crate::messages::Msg;
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
            <a class="nav-link" on_click=|_|{Msg::GoToHomePage}>"Home page"</a>
        </li>
        <li class="nav-item">
                    <a class="nav-link" on_click=|_| {Msg::GoToDocsPage}>"Docs"</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link active" aria-current="page" href="#">"Login"</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" on_click=|_| {Msg::GoToRegister}>"Register"</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" on_click=|_| {Msg::GoToAboutProject}>"About this project"</a>
                </li>
            </ul>
        </div>
    </div>
    </nav>
    <p>"This is homepage" </p>
    <a href="https://www.youtube.com/watch?v=j5eGrWK5XN8&list=PLDtWoQ-cxqiyQgcpmmJ7MFSddekcfL42N&index=53">"Koristen posnetek"</a>
    <a href="https://www.youtube.com/watch?v=j5eGrWK5XN8&list=PLDtWoQ-cxqiyQgcpmmJ7MFSddekcfL42N&index=53">"Isti koristen posnetek"</a>
  </main>
  }}