use sauron::prelude::*;
use crate::messages::{DefinedMsg, GoToPage,SwitchToPageSigned,SwitchToPageUnsigned};
use crate::app::App;
use crate::logics::sidebars;

pub fn view(current_state_of_app : &App) -> Node<DefinedMsg> {
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
            //                    <a class="nav-link" on_click=|_| {DefinedMsg::SetPage(GoToPage::GoToDocsPage)}>"Docs"</a>
            //                </li>
            //            </ul>
            //        </div>
            //    </div>
            //</nav>

            // Fixed Sidebar (desktop)
            {sidebars::left_sidebar(current_state_of_app.current_page.clone())}

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
  <textarea class="form-control" placeholder="Leave a comment here" id="floatingTextarea" on_input=|input|{DefinedMsg::SendComment(input.value())}></textarea>
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
