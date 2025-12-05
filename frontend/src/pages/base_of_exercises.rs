/*
A collection of exercises in some group.
It can also be made 'public' so that even if a user is not part of an 
associated group they can view it (but they won't be able to see other content).
It can also be created independently of groups and be shared across many 
groups.
*/

use sauron::prelude::*;
use crate::messages::Msg;
use crate::experimental_examples::dummy_page_content::dummy_page_content;
pub fn base_of_exercises_display() -> Node<Msg>{
    node!{
        {dummy_page_content()}

}}