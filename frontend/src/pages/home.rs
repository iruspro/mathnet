use sauron::prelude::*;
use crate::messages::Msg;
use sauron::html::{meta,link,title};

pub fn view() -> Node<Msg> {
    html(
        [],
        [
            head(
                [],
                [
                    title([],[text("Home Page")]),
                    meta([name("description"), content("Example of common HTML tags")], []),
                    link([rel("stylesheet"), href("styles.css")], []),
                ],
            ),
            body(
                [],
                [
                    header(
                        [],
                        [
                            h1([], [text("Welcome to the Home Page!")]),
                            nav(
                                [],
                                [
                                    a([href("/")], [text("Home")]),
                                    text(" | "),
                                    a([href("/login")], [text("Login")]),
                                    a([href("/register")], [text("Register")]),
                                    a([href("/docs")],[text("Documentation")]),
                                    // a([href("https://github.com/iruspro/mathnet.git")],[text(GitHub repository)] - nekaj ne dela - kako linkati zunanjo stran

                                ],
                            ),
                        ],
                    ),
                        ]
                    )
                ]
            )
}