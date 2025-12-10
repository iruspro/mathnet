use sauron::Node;
use sauron::prelude::*;
use crate::app::Msg;
use crate::frontend_features::side_elements::{left_sidebar::display_left_sidebar, licence_info::licence_with_left_sidebar_display};
use crate::list_of_pages::Page;
use crate::pages::pages_templates_and_routing::display_content_function::display_content;
use crate::app::App;

pub fn signed_in_page_template(app: &App, current_page: &Page) -> Node<Msg>{
    node!{
        {display_left_sidebar(current_page)}
        <div class="my-content-with-left-sidebar">
        {display_content(current_page, app)}
        </div>
    
        {licence_with_left_sidebar_display()}
}}