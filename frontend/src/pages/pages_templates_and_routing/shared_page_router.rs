/*
Deciding which template to use for pages that are common 
to signed in and signed out states.
*/

use crate::list_of_pages::Page;
use crate::app::App;
use sauron::prelude::*;
use crate::messages::Msg;
use crate::pages::pages_templates_and_routing::{signed_in_page_template::signed_in_page_template, signed_out_page_template::signed_out_page_template};


pub fn shared_page_router(app: &App, page: &Page) -> Node<Msg> {
        if app.is_user_signed_in {
            signed_in_page_template(app, page)
        }
        else {
            signed_out_page_template(app, page)
        }
    
    
}