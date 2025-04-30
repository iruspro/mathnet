use sauron::prelude::*;
use crate::messages::{Msg, GoToPage,SwitchToPageSigned,SwitchToPageUnsigned};

pub fn view() -> Node<Msg> {
    node! {
        <main>
<div class="position-relative">
<div class="position-absolute top-50 start-50 translate-middle">
<div text-align="center">"Changing your user profile data failed. Please try again!"</div>
<button type="button" class="btn btn-primary" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile))}>"Go back to user profile"</button>
</div>
</div>
        </main>
    }}