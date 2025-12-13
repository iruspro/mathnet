/*
A list of projects.
*/

use sauron::prelude::*;
use crate::messages::Msg;
use crate::experimental_examples::dummy_page_content::dummy_page_content;
pub fn project_list_display() -> Node<Msg>{
    node!{
        dummy_page_content()

}}