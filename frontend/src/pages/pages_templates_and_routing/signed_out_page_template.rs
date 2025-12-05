use sauron::Node;
use sauron::prelude::*;
use crate::app::Msg;
use crate::frontend_features::{top_navbar::display_top_navbar};
use crate::list_of_pages::Page;
use crate::pages::pages_templates_and_routing::display_content_function::display_content;



pub fn signed_out_page_template(current_page: &Page) -> Node<Msg>{
    node!{
        {display_top_navbar(current_page)}
        <div class="mycontent">
        {display_content(current_page)}
        </div>
}}