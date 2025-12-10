use crate::messages::Msg;
use sauron::prelude::*;
use crate::frontend_features::date_and_time::date_and_time::display_current_date_and_time;

pub fn dummy_conversation_summary() -> Node<Msg>{
    node!{
 <div class="conversation_message_box">
    <div class="conversation_date_and_time">{text!("Send: {:?}", display_current_date_and_time())}</div>
  </div>
  <div>"Name: John Doe"</div>
  <div class="conversation_message">{text!("{}","lorem ipsum")}</div> 
}
}
    
