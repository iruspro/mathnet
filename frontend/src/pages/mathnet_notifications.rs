/*
A way for MathNet admins to notify users about updates, 
changes, penalties ...
*/



use sauron::prelude::*;
use crate::messages::Msg;
use crate::experimental_examples::dummy_page_content::dummy_page_content;
pub fn mathnet_notifications_display() -> Node<Msg>{
    node!{
        dummy_page_content()

}}