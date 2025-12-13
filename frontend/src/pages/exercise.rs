/*
Exercise statement, solution and discussion. It will be 
shippable in the following sense:
*/

use sauron::prelude::*;
use crate::messages::Msg;
use crate::experimental_examples::dummy_page_content::dummy_page_content;
pub fn exercise_display() -> Node<Msg>{
    node!{
        dummy_page_content()

}}