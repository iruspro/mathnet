use sauron::prelude::*;
use crate::messages::Msg;

pub fn view() -> Node<Msg> {
    node!{
        <main>
        <h1> "Welcome to mathnet!" </h1>
        <input type="button" on_click=|_| {Msg::GoToDocsPage} value="Go to docs"> </input>
        </main>
    }
}