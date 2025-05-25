use crate::messages::{GoToPage, DefinedMsg, SwitchToPageSigned, SwitchToPageUnsigned,SwitchToPageShared};
use sauron::prelude::*;

pub fn view() -> Node<DefinedMsg> {
    node! {
          <main>
            <p>"Error 404: Page not found."</p>
          </main>
    }}