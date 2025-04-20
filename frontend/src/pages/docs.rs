use sauron::prelude::*;
use crate::messages::Msg;
use sauron::html::{meta,title,link};

pub fn view() -> Node<Msg> {
    node!{
        <main>
        <h1> "Welcome to mathnet!" </h1>
       <a href="https://github.com/iruspro/mathnet.git"> "Go to official GitHub Repository" </a>
        </main>
    }
}