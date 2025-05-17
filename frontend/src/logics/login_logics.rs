pub use crate::app::App;
pub use crate::list_of_pages::{Page, SignedPage, UnsignedPage};
use crate::web_sys::console;



pub fn check_login_attempt_validity(app : &mut App){
    console::log_1(&"Hello from Rust!".into());
    app.current_page = Page::ItemSignedPage(SignedPage::GroupsList);
    console::log_1(&"Hello from Rust2!".into());
}