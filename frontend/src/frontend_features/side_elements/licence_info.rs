/*
Licence footer.
*/

use sauron::prelude::*;
use crate::messages::Msg;

pub fn licence_without_left_sidebar_display() -> Node<Msg>{
    node!{
        <div class="myfooter">
        "Notification:"
        <a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-nc-nd/4.0/88x31.png" /></a><br />"This work is licensed under a" <a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/">"Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International License"</a>.
        </div>
    }

}

pub fn licence_with_left_sidebar_display() -> Node<Msg>{
    node!{
        <div class="myfooter-with-left-sidebar">
        "Notification:"
        <a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-nc-nd/4.0/88x31.png" /></a><br />"This work is licensed under a" <a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/">"Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International License"</a>.
        </div>
    }

}