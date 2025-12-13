pub use crate::app::App;
pub use crate::list_of_pages::{Page,SignedPage,UnsignedPage};

pub fn check_registration_validity(usr : &mut App){
    usr.current_page = Page::PageSortUnsigned(UnsignedPage::Home);
}