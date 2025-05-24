use crate::experimental_examples::imaginary_chat_messages::dummy_chat_messages;
pub use crate::messages::*;
pub use sauron::Node;

pub use crate::app::App;
pub use crate::communication_with_server::get_conversation::get_whole_conversation_from_server;
pub use crate::logics::date_and_time;
pub use crate::structs::chat_message::{ChatId, ChatMessage};
pub use crate::structs::friends_at_sidebar::{FriendAtSidebar, ShowChats};
pub use crate::structs::user::{User, UserId, new};
pub use sauron::{node, text};

pub fn post_new_message() -> Node<DefinedMsg> {
    node! {
                                    <div class="card mb-3" style="max-width: 540px;">
      <div class="row g-0">
        <div class="col-md-4">
        <form>
        <div class="mb-3">
        <input type="text"
        class="form-control"
        id="exampleFormControlInput1"
        placeholder= "Send new message"
        on_input=|input|{DefinedMsg::SendConversationMessage(input.value())}></input>
      </div>
    </form>
        </div>
      </div>
    </div>
    }
}
