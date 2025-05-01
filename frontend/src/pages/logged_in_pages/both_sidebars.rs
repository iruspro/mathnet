use sauron::prelude::*;
use crate::messages::{Msg, GoToPage, SwitchToPageSigned};

pub fn view() -> Node<Msg> {
    node! {
        <main>
            // LEFT SIDEBAR
            <div class="sidebar d-none d-md-block text-white">
                <h4>"Sidebar"</h4>
                <ul class="nav flex-column">
                    <li class="nav-item">
                        <a class="nav-link text-white" href="#">"Left 1"</a>
                    </li>
                </ul>
            </div>

            // RIGHT SIDEBAR (NEW)
            <div class="right-sidebar d-none d-md-block text-white">
                <h4>"Right Sidebar"</h4>
                <ul class="nav flex-column">
                    <li class="nav-item">
                        <a class="nav-link text-white" href="#">"Right 1"</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link text-white" href="#">"Right 2"</a>
                    </li>
                </ul>
            </div>

            // MAIN CONTENT
            <div class="content">
                <div class="container-fluid">
                    <div class="row">
                        <div class="col-12">
                            <h1>"Main Content"</h1>
                            <p>"This is the main area between the sidebars."</p>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}
