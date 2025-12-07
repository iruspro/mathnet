/*
Useful learning resources. Find 
published base_of_learning_resources
*/



use sauron::prelude::*;
use crate::messages::Msg;
use crate::experimental_examples::dummy_page_content::dummy_page_content;
pub fn learning_resources_display() -> Node<Msg>{
    node!{
        {dummy_page_content()}

}}