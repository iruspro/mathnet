use sauron::prelude::*;
use crate::messages::{Msg, GoToPage,SwitchToPageSigned,SwitchToPageUnsigned};

pub fn view() -> Node<Msg> {
    node! {
        <main>
            // Top Navbar
            //<nav class="navbar navbar-expand navbar-dark bg-dark fixed-top">
            //    <div class="container-fluid">
            //        <a class="navbar-brand" href="#">"MathNet"</a>
            //        <button class="navbar-toggler" type="button" data-bs-toggle="offcanvas" data-bs-target="#offcanvasSidebar">
            //            <span class="navbar-toggler-icon"></span>
            //        </button>
            //        <div class="collapse navbar-collapse">
            //            <ul class="navbar-nav me-auto mb-2 mb-lg-0">
            //                <li class="nav-item">
            //                    <a class="nav-link active" aria-current="page" href="#">"Home page"</a>
            //                </li>
            //                <li class="nav-item">
            //                    <a class="nav-link" on_click=|_| {Msg::SetPage(GoToPage::GoToDocsPage)}>"Docs"</a>
            //                </li>
            //            </ul>
            //        </div>
            //    </div>
            //</nav>

            // Fixed Sidebar (desktop)
            <div class="sidebar d-none d-md-block text-white">
                <h4>"Sidebar"</h4>
                <ul class="nav flex-column">
                <li class="nav-item">
                <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile))}>"User profile"</a>
            </li>
            <li class="nav-item">
                <a class="nav-link text-white" href="#">"Groups"</a>
            </li>
            <li class="nav-item">
                <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToDocsPage))} >"Docs"</a>
            </li>
            <li class="nav-item">
                <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToAboutProject))}>"About project"</a>
            </li>
            <li class="nav-item">
                <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToPrivacyAndSecurity))}>"Privacy and security"</a>
            </li>
            <li class="nav-item">
                <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToSettings))}>"Settings"</a>
            </li>
            <li class="nav-item">
                <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToLogOut))}>"Log out"</a>
            </li>
                </ul>
            </div>

            // Offcanvas Sidebar (mobile)
            <div class="offcanvas offcanvas-start bg-dark text-white" tabindex="-1" id="offcanvasSidebar">
                <div class="offcanvas-header">
                    <h5 class="offcanvas-title">"Sidebar"</h5>
                    <button type="button" class="btn-close btn-close-white" data-bs-dismiss="offcanvas"></button>
                </div>
                <div class="offcanvas-body">
                    <ul class="nav flex-column">
                        <li class="nav-item">
                            <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile))}>"User profile"</a>
                        </li>
                        <li class="nav-item">
                        <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroupsList))}>"Groups"</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToDocsPage))} >"Docs"</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToAboutProject))}>"About project"</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToPrivacyAndSecurity))}>"Privacy and security"</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToSettings))}>"Settings"</a>
                        </li>
                        <li class="nav-item">
                        <a class="nav-link text-white" href="#">"Suggest us"</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToLogOut))}>"Log out"</a>
                        </li>
                    </ul>
                </div>
            </div>

            // Main Content
            <div class="content">
                <div class="container-fluid">
                    <div class="row">
                        <div class="col-12">
                            <h1 class="text-center">"Your groups"</h1>
                            <div class="my_card_design">
                            <div class="card border-dark mb-3" >
                            <div class="card text-center">
                              <div class="card-body">
                                <h5 class="card-title">"Your suggestion"</h5>
                                <p>"If you want to let us know about your opinion about MathNet or you have any suggestion for further development or you have any issues using this page or you found a bug, please send us message!
                                The message won't be anonymous. We will reply you as soon as possible"</p>
                                <p>"If you are familiar to using GitHub, you can also visit this project's GitHub repository and mark comments and issues in 'Issues' section. Link to the repository is available "<a href="https://github.com/iruspro/mathnet.git" class="btn btn-primary stretched link">"here"</a>.</p>
                                <form>
                                <div class="d-grid gap-2 col-6 mx-auto">
                                <div class="form-floating">
  <textarea class="form-control" placeholder="Leave a comment here" id="floatingTextarea" on_input=|input|{Msg::SendComment(input.value())}></textarea>
  <label for="floatingTextarea">"Comments"</label>
</div>
</div>
                                </form>

</div>
</div>
</div>
</div>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}
