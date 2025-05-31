// region:    --- Modules
use sauron::{
    Node,
    html::{attributes::*, *},
};
// endregion: --- Modules

pub fn info_footer() -> Node<()> {
    footer(
        [],
        [p(
            [],
            [
                text("Written by "),
                a(
                    [href("https://github.com/iruspro/"), target("_blank")],
                    [text("Ruslan Urazbakhtin")],
                ),
                text(" and "),
                a(
                    [
                        href("https://github.com/Benjamin-Dagarin"),
                        target("_blank"),
                    ],
                    [text("Benjamin Dagarin")],
                ),
            ],
        )],
    )
}
