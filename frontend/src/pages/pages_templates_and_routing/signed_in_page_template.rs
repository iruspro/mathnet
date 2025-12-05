use sauron::Node;
use sauron::prelude::*;
use crate::app::Msg;
use crate::frontend_features::{left_sidebar::display_left_sidebar};
use crate::list_of_pages::Page;
use crate::pages::pages_templates_and_routing::display_content_function::display_content;

pub fn signed_in_page_template(current_page: &Page) -> Node<Msg>{
    node!{
        {display_left_sidebar(current_page)}
        <div class="my-content-with-left-sidebar">
        {display_content(current_page)}
        </div>
}}