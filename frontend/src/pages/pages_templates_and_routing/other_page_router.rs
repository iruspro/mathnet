
use crate::list_of_pages::Page;
use crate::app::App;
use crate::pages::pages_templates_and_routing::display_content_function::display_content;
use sauron::prelude::*;
use crate::messages::Msg;

pub fn other_page_router(app: & App, page: &Page) -> Node<Msg> {
    display_content(page)

    
}