pub use crate::app::App;
pub use crate::list_of_pages::{Page, SignedPage, UnsignedPage};


pub fn check_login_attempt_validity(app : &mut App){
    app.current_page = Page::ItemSignedPage(SignedPage::GroupsList)
}