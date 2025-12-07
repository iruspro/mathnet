/*
Projects to work on groups. If there was a project of building book library system, 
programmers who would operate would join this 'project'.
*/

use sauron::prelude::*;
use crate::messages::Msg;
use crate::experimental_examples::dummy_page_content::dummy_page_content;
pub fn project_display() -> Node<Msg>{
    node!{
        {dummy_page_content()}

}}