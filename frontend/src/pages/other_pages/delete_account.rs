use crate::messages::{GoToPage, DefinedMsg, SwitchToPageSigned, SwitchToPageUnsigned};
use sauron::prelude::*;

pub fn view() -> Node<DefinedMsg> {
    node! {
            <main>
    <div class="position-relative">
    <div class="position-absolute top-50 start-50 translate-middle">
    <div text-align="center">"Are you sure you want to delete yout account? Once deleted, we will be unable to recover your data"</div>
    <button type="button" class="btn btn-primary" on_click=|_|{DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile))}>"Cancel deletion"</button>
    <button type="button" class="btn btn-danger" on_click=|_|{DefinedMsg::SetPage(GoToPage::GoToPageUnsigned(SwitchToPageUnsigned::GoToHomePage))}>"Confirm deletion"</button>
    </div>
    </div>
            </main>
        }
}
