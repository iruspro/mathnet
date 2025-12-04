use sauron::Node;

use crate::app::Msg;
use crate::frontend_features::{top_navbar::display_top_navbar};



pub fn logged_out_page_template(current_page: &Page) -> Node<Msg>{
    node!{
        {display_top_navbar(current_page)}
        <div class="mycontent">
    // TODO: add content.
        </div>
}}