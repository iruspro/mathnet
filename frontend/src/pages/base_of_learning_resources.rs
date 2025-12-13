/*
Groups could also have a base of study resources to share. It would be collected so that
everyone can easiy find what they want to.
It can also be assocciated to individual persons.
*/

use sauron::prelude::*;
use crate::messages::Msg;
use crate::experimental_examples::dummy_page_content::dummy_page_content;
pub fn base_of_learning_resources_display() -> Node<Msg>{
    node!{
        dummy_page_content()

}}