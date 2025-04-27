pub use crate::app::App;
pub use crate::list_of_pages::Page;


pub fn check_login_attempt_validity(app : &mut App){
    app.current_page = Page::GroupsList
}