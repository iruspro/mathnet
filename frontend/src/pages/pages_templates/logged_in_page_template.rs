use sauron::Node;

use crate::app::Msg;
use crate::frontend_features::{left_sidebar::display_left_sidebar};



pub fn logged_in_page_template(current_page: &Page) -> Node<Msg>{
    node!{
        {display_left_sidebar(current_page)}
        <div class="my-content-with-left-sidebar">
    // TODO: add content.
        </div>
}}