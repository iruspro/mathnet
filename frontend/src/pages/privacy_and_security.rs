/*
Info about privacy and security of MathNet usage.
*/


use sauron::prelude::*;
use crate::messages::Msg;
use crate::experimental_examples::dummy_page_content::dummy_page_content;
pub fn privacy_and_security_display() -> Node<Msg>{
    node!{
        {dummy_page_content()}

}}