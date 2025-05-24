use crate::experimental_examples::imaginary_chat_messages::dummy_chat_messages;
pub use crate::messages::*;
pub use sauron::Node;

pub use crate::app::App;
pub use crate::communication_with_server::get_conversation::get_whole_conversation_from_server;
pub use crate::list_of_pages::Page;
pub use crate::logics::{date_and_time, post_new_message, sidebars};
pub use crate::structs::chat_message::{ChatId, ChatMessage};
pub use crate::structs::friends_at_sidebar::{FriendAtSidebar, ShowChats};
pub use crate::structs::user::{User, UserId, new};
pub use sauron::{node, text};

/* When you click on some person in show_chats_in_content, you get the selected conversation.*/
pub fn show_conversation(chat_id: ChatId, current_page: Page) -> Node<DefinedMsg> {
    let tuple_of_vec_of_chat_messages_and_friend_name = get_whole_conversation_from_server(chat_id);
    let friend_name: String = tuple_of_vec_of_chat_messages_and_friend_name.1;
    let vec_of_chat_messages_unused: Vec<ChatMessage> =
        tuple_of_vec_of_chat_messages_and_friend_name.0;
    let vec_of_chat_messages = dummy_chat_messages();

    node! {
            <main>
                // Left Sidebar (Desktop)
                 {sidebars::left_sidebar_special(current_page.clone())}

                // Right Sidebar
                <div class="right-sidebar d-none d-md-block text-white">
                    <h4>"Right Sidebar"</h4>
                    <ul class="nav flex-column">
                    <div class="search-container">
            <input type="text" class="form-control search-input" placeholder="Find a person" on_input=|input|{DefinedMsg::SearchFriend(input.value())}></input>
            <i class="fas fa-search search-icon"></i>
          </div>
                        //{show_friends_at_sidebar()};
                    </ul>
                </div>

                // Main Content
                <div class="content">
                    <div class="container-fluid"> //scrollable-container doesn't do anything because sidebars are already fixed (independent from scrolling the content.)
                        <h1 class="text-center">{text!("{}",friend_name)}</h1>
    <div class="d-grid gap-2 d-md-flex justify-content-md-start">
      <button class="btn btn-primary" type="button" on_click=|_| {DefinedMsg::SetConversationToNone}>"Go back to all messages"</button>
      </div>
                            {for chat_message in vec_of_chat_messages{
                                node!{
                                    <div class="card mb-3" style="max-width: 540px;">
      <div class="row g-0">
        <div class="col-md-4">
          {text!("{}",chat_message.sender)}
        </div>
        <div class="col-md-8">
          <div class="card-body">
            <p class="card-text"><small class="text-body-secondary">{text!("{}",chat_message.date_and_time.date_and_time_to_string())}</small></p>
            <p class="card-text">text!("{}", chat_message.content)</p>
          </div>
        </div>
      </div>
    </div>
    /*
                <div class="card">
                <div class="card-body">
                <p>{text!{"{}",chat_message.date_and_time.date_and_time_to_string()}}</p>
                <p>{text!{"{}",chat_message.content}}</p>
                <p></p>
                </div>
                </div>
    */                      }

                            }}
                    </div>
    {post_new_message::post_new_message()}
                </div>
            </main>
        }
}
