/*
Are you sure you want to delete your account? 
*/

use sauron::prelude::*;
use crate::messages::{Msg};
use crate::list_of_pages::{Page, UnsignedPage, SignedPage};

pub fn delete_account_display() -> Node<Msg> {
    node! {
        <main>
<div class="position-relative">
<div class="position-absolute top-50 start-50 translate-middle">
<div text-align="center">"Are you sure you want to delete yout account? Once deleted, we will be unable to recover your data"</div>
<button type="button" class="btn btn-primary" on_click=|_|{Msg::SetPage(Page::PageSortSigned(SignedPage::UserProfile))}>"Cancel deletion"</button>
<button type="button" class="btn btn-danger" on_click=|_|{Msg::DeleteAccount}>"Confirm deletion"</button>
</div>
</div>
        </main>
    }}