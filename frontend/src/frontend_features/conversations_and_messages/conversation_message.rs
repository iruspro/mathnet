pub use crate::structs::user;
pub use crate::structs::emojis_and_reactions;
pub use crate::logics::date_and_time::{DateAndTime, display_current_date_and_time};
pub use crate::messages::Msg;
use sauron::prelude::*;



pub struct ConversationChannel{
    Conversation_id : u128,
    Conversation_messages_vec : Vec<ConversationMessage>,
}


/* ConversationMessage represents a message that some person can 
send to other person via personal conversation */
pub struct ConversationMessage{
    pub Conversation_id: u128,
    pub Conversation_message_id : u128,
    pub date_and_time : DateAndTime,
    pub content: String,
    pub latex_on : bool,
    pub sender: String,
    pub reactions : Vec<emojis_and_reactions::Emoji>,

}

// Implement that!
impl ConversationMessage{
    pub fn new() -> ConversationMessage{
        ConversationMessage { Conversation_id: 123, Conversation_message_id: 123, date_and_time: display_current_date_and_time(), content: "Lorem ipsum".to_string(), latex_on: false, sender: "JohnDoe".to_string(), reactions: Vec::new()}
    }
    pub fn conversation_message_display(&self) -> Node<Msg>{
     node!{   <div class="card mb-3" style="max-width: 540px;">
  <div class="row g-0">
    <div class="col-md-4">
      {text!("{}",self.sender)}
    </div>
    <div class="col-md-8">
      <div class="card-body">
        <p class="card-text"><small class="text-body-secondary">{text!("{}",self.date_and_time.date_and_time_to_string())}</small></p>
        <p class="card-text">text!("{}", self.content)</p>
      </div>
    </div>
  </div>
</div>

    }}
}