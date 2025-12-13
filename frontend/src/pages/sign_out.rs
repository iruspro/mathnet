use sauron::prelude::*;
use crate::messages::{Msg};
use crate::list_of_pages::{Page::{PageSortSigned, PageSortUnsigned}, SignedPage, UnsignedPage};
use crate::list_of_pages::Page;

pub fn sign_out_view() -> Node<Msg> {
    node! {
        <main>
<div class="position-relative">
<div class="position-absolute top-50 start-50 translate-middle">
<div text-align="center">"Are you sure that you want to log out?"</div>
<button type="button" class="btn btn-primary" on_click=|_|{Msg::SetPage(Page::PageSortSigned(SignedPage::UserProfileManagement))}>"Stay logged in"</button>
<button type="button" class="btn btn-danger" on_click=|_|{Msg::SetPage(Page::PageSortUnsigned(UnsignedPage::Home))}>"Sign out"</button>
</div>
</div>
        </main>
    }}