/*
Groups are chats with three or more people. Not sure whether to use this name 
for larger or smaller social 'groups' or both. 
I think the best option is to use 'group' for Discord like 'servers' and 
some other naming for more intimate 'groups' of people.
*/

use sauron::prelude::*;
use crate::messages::Msg;
use crate::experimental_examples::dummy_page_content::dummy_page_content;
pub fn group_display() -> Node<Msg>{
    node!{
        {dummy_page_content()}

}}