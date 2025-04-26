use sauron::prelude::*;
use crate::messages::Msg;
use sauron::html::{meta,title,link};

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
                            h1([], [text("Welcome to Chat!")]),
                            
                        ],
                    ),
                        ]
                    )
                ]
            )
}