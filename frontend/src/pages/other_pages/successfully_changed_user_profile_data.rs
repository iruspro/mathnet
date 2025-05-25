use crate::messages::{GoToPage, DefinedMsg, SwitchToPageSigned, SwitchToPageUnsigned};
use sauron::prelude::*;
use crate::structs::user::UserId;

pub fn view(user_id : UserId) -> Node<DefinedMsg> {
    node! {
            <main>
    <div class="position-relative">
    <div class="position-absolute top-50 start-50 translate-middle">
    <div text-align="center">"You successfully changed user profile data."</div>
    <button type="button" class="btn btn-primary" on_click=move|_|{DefinedMsg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile(user_id.clone())))}>"Go back to user profile"</button>
    </div>
    </div>
            </main>
        }
}
