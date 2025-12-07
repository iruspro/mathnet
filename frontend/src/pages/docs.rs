/*
How to use MathnNet, link to GitHub repository
*/
use sauron::prelude::*;
use crate::messages::Msg;

pub fn docs_display() -> Node<Msg> {
    node!{
        <h1>
            "Documentation and how to use MathNet"
        </h1>

        <p>
        "Here is the link to" <a href="https://github.com/iruspro/mathnet.git">"MathNet's GitHub page"</a>. "Be sure you are 
        viewing the main branch."
        </p>

        <button on_click=|_|{Msg::NoAction}> "Dummy button" </button>

        <p>" And here is some lorem ipsum to fill the page: 
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas finibus, purus sed consequat facilisis, ipsum urna dapibus tortor, vitae aliquet enim lorem vel ex. Vestibulum eu ultrices nibh, ut sollicitudin lacus. Vestibulum vitae justo vel libero aliquam mollis in non mauris. Nullam non orci a neque molestie ornare. Pellentesque euismod odio libero, vitae eleifend leo elementum eu. Curabitur rhoncus vestibulum augue a ullamcorper. Suspendisse sit amet finibus quam, eu ultrices magna. Aliquam eu finibus mi. In lobortis sagittis odio, ut efficitur justo dictum quis.

        Nullam tempor ac enim sit amet molestie. Duis lacinia ligula non imperdiet sodales. Donec massa purus, faucibus vel orci varius, ornare aliquet lectus. Aenean vel egestas dolor. Nunc tempor quam lectus, ut rhoncus sem mattis vitae. Praesent facilisis tempor felis ut tempus. Nulla facilisi. Interdum et malesuada fames ac ante ipsum primis in faucibus. Suspendisse potenti. Curabitur at leo ante. Pellentesque in lacus posuere, congue enim et, tristique purus. Nulla facilisi.

        Vivamus at tincidunt sapien. Morbi lacus dolor, malesuada id quam eu, egestas bibendum nulla. Aliquam erat volutpat. Etiam efficitur dui a leo posuere, quis sodales libero interdum. Fusce sed viverra augue. Duis sit amet elementum augue, blandit sagittis nisl. Suspendisse scelerisque accumsan risus vel elementum. Suspendisse in elit vel dui egestas finibus. Duis efficitur mattis efficitur. Vestibulum vestibulum eu ex vel varius.

        Integer consequat maximus turpis a vehicula. Mauris sem erat, tristique a sollicitudin id, porttitor sit amet nibh. Sed sed tortor ut ex ultricies tempus at id arcu. Nunc commodo massa odio, eu tincidunt nulla auctor at. Sed eu massa id magna elementum aliquam. Etiam consectetur diam lacus, ut viverra purus condimentum vitae. Maecenas dignissim fringilla nibh, ut sollicitudin felis lacinia nec. Sed iaculis, urna at eleifend ullamcorper, tortor sapien commodo nisi, efficitur facilisis enim mi nec ante. Curabitur tempus nulla sit amet lacus sodales mattis. Integer libero nulla, accumsan in ullamcorper ac, venenatis eget quam. Praesent feugiat, sapien sit amet efficitur tempor, libero lorem lacinia nunc, at tristique turpis leo at enim.

        Nunc maximus a velit at dignissim. Morbi vulputate nisi nec enim facilisis, sit amet semper leo aliquet. Nulla pretium urna mauris. Curabitur imperdiet molestie vulputate. Maecenas pulvinar pulvinar ligula non laoreet. Mauris consectetur aliquam diam in commodo. Nam ac lacinia nulla. Suspendisse malesuada nibh id nisi congue, vel convallis dui convallis. Integer malesuada non ex eget bibendum. In hac habitasse platea dictumst. Ut elementum mi leo, vestibulum viverra eros auctor ac. Suspendisse potenti. Morbi vestibulum lectus hendrerit quam iaculis ornare. 
        " </p>
}}