/*
Various settings
*/


use sauron::prelude::*;
use crate::messages::Msg;
use crate::experimental_examples::dummy_page_content::dummy_page_content;
pub fn settings_display() -> Node<Msg>{
    node!{
        dummy_page_content()

}}