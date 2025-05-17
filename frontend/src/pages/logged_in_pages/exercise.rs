use std::thread::current;

use sauron::prelude::*;
use crate::messages::{Msg, GoToPage,SwitchToPageSigned,SwitchToPageUnsigned};
use crate::app::App;
use crate::logics::sidebars;

pub fn view(current_state_of_app : &App) -> Node<Msg> {
    node! {
        <main>
            // Top Navbar
            //<nav class="navbar navbar-expand navbar-dark bg-dark fixed-top">
            //    <div class="container-fluid">
            //        <a class="navbar-brand" href="#">"MathNet"</a>
            //        <button class="navbar-toggler" type="button" data-bs-toggle="offcanvas" data-bs-target="#offcanvasSidebar">
            //            <span class="navbar-toggler-icon"></span>
            //        </button>
            //        <div class="collapse navbar-collapse">
            //            <ul class="navbar-nav me-auto mb-2 mb-lg-0">
            //                <li class="nav-item">
            //                    <a class="nav-link active" aria-current="page" href="#">"Home page"</a>
            //                </li>
            //                <li class="nav-item">
            //                    <a class="nav-link" on_click=|_| {Msg::SetPage(GoToPage::GoToDocsPage)}>"Docs"</a>
            //                </li>
            //            </ul>
            //        </div>
            //    </div>
            //</nav>

            // Fixed Sidebar (desktop)
            /* 
            <div class="sidebar d-none d-md-block text-white">
                <h4>"Sidebar"</h4>
                <ul class="nav flex-column">
                <li class="nav-item">
                <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile))}>"User profile"</a>
            </li>
            <li class="nav-item">
                <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroupsList))}>"Chat with friends"</a>
            </li>
            <li class="nav-item">
                <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToGroupsList))}>"Groups"</a>
            </li>
            <li class="nav-item">
                <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToDocsPage))} >"Docs"</a>
            </li>
            <li class="nav-item">
                <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToAboutProject))}>"About project"</a>
            </li>
            <li class="nav-item">
                <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToPrivacyAndSecurity))}>"Privacy and security"</a>
            </li>
            <li class="nav-item">
                <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToSettings))}>"Settings"</a>
            </li>
            <li class="nav-item">
                <a class="nav-link text-white" href="#">"Notifications"</a>
            </li>
            <li class="nav-item">
                <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToLogOut))}>"Log out"</a>
            </li>
                </ul>
            </div>

            // Offcanvas Sidebar (mobile)
            <div class="offcanvas offcanvas-start bg-dark text-white" tabindex="-1" id="offcanvasSidebar">
                <div class="offcanvas-header">
                    <h5 class="offcanvas-title">"Sidebar"</h5>
                    <button type="button" class="btn-close btn-close-white" data-bs-dismiss="offcanvas"></button>
                </div>
                <div class="offcanvas-body">
                    <ul class="nav flex-column">
                        <li class="nav-item">
                            <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToUserProfile))}>"User profile"</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToChatWithFriends))}>"Chat with friends"</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToDocsPage))} >"Docs"</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToAboutProject))}>"About project"</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToPrivacyAndSecurity))}>"Privacy and security"</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToSettings))}>"Settings"</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToLogOut))}>"Log out"</a>
                        </li>
                    </ul>
                </div>
            </div>
            */
            {sidebars::left_sidebar(current_state_of_app.current_page.clone())}

            // Main Content
            <div class="content">
                <div class="container-fluid">
                    <div class="row">
                        <div class="col-10">
                            <h1 class="text-center">"Your groups"</h1>
                            <p class="basicparagraph text-start">
                                "Groups you belong to.
                                

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras luctus consectetur placerat. Donec non pretium sapien. Donec ac placerat ex. Aenean tempus massa nulla, nec ullamcorper leo tempus eget. Vestibulum at lectus ut libero ullamcorper consectetur. Vestibulum auctor urna venenatis libero eleifend, in sodales odio dictum. Curabitur auctor, massa eget ultricies efficitur, justo nulla porta purus, in laoreet quam turpis ut risus. Aliquam nunc nibh, placerat eget bibendum varius, porta eget tellus. Phasellus in lacinia augue, sed consequat ex. Curabitur laoreet mi nec eros tristique, ac tristique mauris tempus.

Pellentesque in lacus laoreet metus ultrices egestas in sit amet justo. Ut tempor lectus molestie tortor egestas egestas. Praesent vel pharetra nisi, ut convallis felis. Duis a feugiat velit, congue pretium ex. Curabitur vel fringilla ligula. Phasellus malesuada libero vitae ex ornare, eget imperdiet ex auctor. Etiam sit amet ex lectus. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Pellentesque sed tincidunt ligula. Nulla a dui vitae leo tempor efficitur. Vivamus vel nisi eget sapien blandit molestie ac quis felis.

Quisque fringilla purus eget libero molestie, eget tempus ligula consectetur. Sed vulputate porttitor ante, sit amet aliquet massa viverra quis. Suspendisse varius sapien id arcu tristique aliquam. Sed eu interdum turpis. Vivamus finibus lectus nulla. Maecenas erat mi, dictum sed aliquet eu, tincidunt non nibh. Etiam lectus libero, volutpat ac malesuada at, sodales ac mauris. Pellentesque vitae ultrices erat, vel faucibus odio. Ut porta cursus erat, eu fringilla ante aliquam id. Fusce bibendum feugiat neque, nec volutpat enim ultrices vitae. Vestibulum in orci mi.

Nunc viverra mi eget nunc vulputate, ut aliquam neque vestibulum. Quisque porttitor mattis ipsum non ullamcorper. Suspendisse eu orci ullamcorper, semper quam nec, aliquam ligula. Sed in eros id sem feugiat pellentesque. Donec magna mauris, blandit et nisl in, semper aliquam odio. Mauris sed ligula lacus. Cras dignissim, neque eu convallis vehicula, ligula orci eleifend nulla, in commodo purus nisl vel risus. Duis tristique hendrerit risus sit amet suscipit. Donec ligula tellus, scelerisque ullamcorper posuere nec, porttitor molestie libero. Morbi pharetra congue ante, eu tristique velit congue rutrum.

Etiam ac rutrum nulla. Etiam dictum metus mi, a sollicitudin risus rhoncus iaculis. Duis placerat dignissim felis nec sollicitudin. Sed commodo blandit commodo. Nulla at eros nisi. Ut pellentesque efficitur nulla, id egestas elit dapibus vel. Vivamus neque eros, imperdiet eget fringilla nec, consectetur sit amet libero. Vivamus sit amet congue turpis. Sed metus est, imperdiet quis sollicitudin nec, suscipit in purus. Sed suscipit convallis interdum. Vestibulum ullamcorper pretium fermentum. Praesent aliquam hendrerit felis et elementum. Donec sagittis odio mattis ante dictum, et feugiat dolor pretium. Pellentesque in sagittis libero. Morbi eu neque nisl.

Vivamus bibendum sollicitudin facilisis. Morbi tempus accumsan massa, quis rhoncus lectus ultricies ac. Pellentesque ut dui sit amet elit maximus facilisis ut a lectus. Ut non tincidunt mauris. Aenean vel mi dictum, eleifend tellus a, convallis mauris. Morbi luctus mauris augue, sed bibendum sem euismod sit amet. Etiam at dolor in dui maximus ornare. Integer mattis massa ac elit vulputate venenatis. Sed volutpat, sem ut luctus sagittis, dui massa interdum quam, in tempor est est nec augue. Ut dapibus euismod lacus eu hendrerit.

Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Maecenas non venenatis magna. In consequat diam eu urna pretium congue. Suspendisse ut mi imperdiet mauris iaculis porttitor et id risus. Suspendisse imperdiet, mauris eu malesuada bibendum, ante arcu commodo neque, nec malesuada odio odio id turpis. Donec pretium a felis id lobortis. Morbi consequat odio velit, nec molestie tellus consequat ac. Donec vitae ultricies massa.

Nam tempor purus vitae vulputate sagittis. Sed et magna ut dolor tempus volutpat vel id diam. Morbi finibus lorem eu erat molestie euismod. Suspendisse tincidunt lectus risus, eget malesuada mi accumsan sed. Maecenas et consequat ligula. Vivamus bibendum dictum neque ac mattis. Phasellus nec volutpat nunc, at ornare enim. Phasellus eget venenatis dolor. Curabitur gravida ipsum massa, a tempor nibh efficitur ultrices. Quisque non bibendum odio. Sed tincidunt felis eu volutpat tempus. Aliquam malesuada lobortis felis.

Aliquam ac mi non diam aliquet vulputate quis a metus. Integer mauris sem, condimentum eget nisi a, euismod tempus diam. Curabitur consequat suscipit elit in fringilla. Vivamus ut placerat tellus, rutrum lobortis eros. Suspendisse et metus arcu. Ut non pulvinar felis. Etiam vel varius urna, et efficitur nisl. Donec varius metus at justo pretium venenatis. Etiam eu tortor a eros luctus tincidunt at vel neque. Nunc vitae urna mi. Ut quam leo, fermentum eget lorem sit amet, molestie interdum risus.

Vivamus accumsan massa eu tincidunt gravida. Proin eget consequat magna, at lacinia odio. Nullam venenatis, eros eget tincidunt tempor, erat lorem faucibus dui, vitae dignissim risus tellus sed odio. Sed rhoncus cursus suscipit. Curabitur urna orci, sodales consectetur imperdiet ac, commodo vel quam. Nam ut accumsan est. Vestibulum quis lectus metus. Curabitur sit amet ultricies velit. Nullam aliquam commodo sapien ac sodales.

Sed malesuada ut dolor sed euismod. Proin et consectetur felis, at finibus libero. In nunc ex, tempor sed imperdiet ut, sagittis nec ante. Morbi nec ante vitae ex tristique cursus. Aliquam at nisi dui. Donec malesuada, ipsum vitae ultrices ultrices, massa ante faucibus odio, ac placerat dui lectus quis urna. Sed ornare enim non velit placerat, quis auctor libero tempor. Aliquam leo est, lobortis nec mollis ut, pretium eu nulla. Duis scelerisque pretium libero vel iaculis. Etiam lobortis neque urna, ac vehicula massa tempus vitae. Proin rhoncus quam ac velit luctus, et sagittis diam accumsan.

Proin mi nunc, sagittis sed tristique a, commodo eleifend neque. Pellentesque id orci feugiat ex aliquam finibus in nec nisl. Curabitur suscipit condimentum enim, vel pretium mauris sollicitudin in. Pellentesque ornare lobortis faucibus. Integer at blandit metus. Etiam suscipit tempus molestie. Phasellus varius, mauris quis sollicitudin viverra, ante orci luctus quam, eget scelerisque nunc massa non enim. Donec at metus tincidunt, suscipit elit eget, efficitur leo. Mauris convallis ultrices ipsum at euismod. Donec ut auctor urna, sed gravida massa.

Integer nec eros ipsum. Sed nec lobortis risus, a malesuada dolor. Fusce lobortis mauris risus, et lobortis ante malesuada in. Morbi fermentum risus lacinia, dignissim arcu ac, dictum augue. Quisque ut ligula at orci aliquam tristique. Aliquam mollis scelerisque rhoncus. Aliquam ac augue feugiat urna pharetra ultricies vitae a urna. Fusce imperdiet consequat erat sit amet fermentum. Vivamus interdum consectetur pellentesque. Nullam semper mauris nisl, non fermentum augue gravida nec. Pellentesque a dignissim nisl, id aliquet nisl. Sed mauris nibh, auctor ultricies ante quis, mattis rutrum felis. Pellentesque ipsum augue, lacinia a augue quis, laoreet viverra ligula. Mauris non suscipit quam, ac dapibus arcu.

Quisque varius euismod tincidunt. Fusce diam risus, auctor cursus molestie quis, porta ut metus. Etiam ac lacus non erat accumsan laoreet ac in arcu. Nam fermentum gravida egestas. Phasellus sit amet magna eu quam fringilla ultrices eu non massa. Curabitur vulputate, lacus non viverra finibus, tortor tellus bibendum turpis, faucibus iaculis ligula magna non est. Vestibulum faucibus maximus nisl, at hendrerit metus placerat cursus. Praesent pretium semper metus. Cras sit amet malesuada quam, vel efficitur tellus. Duis ultricies leo nec lorem faucibus aliquam. Sed a lorem laoreet, pretium nisi at, semper urna. Nulla vel massa justo. Praesent dapibus arcu orci, id porttitor ipsum consectetur vehicula. Integer aliquam, mauris sed pellentesque ornare, nunc urna sagittis dolor, id vestibulum libero velit ut lorem. Etiam ut faucibus augue. Nullam posuere arcu id justo pellentesque, eget vestibulum nunc tincidunt.

Vestibulum at justo at velit tempor ullamcorper. Duis rutrum elit neque, a elementum lorem placerat id. Aenean purus tortor, posuere vel suscipit ut, vulputate vel mauris. Sed mauris est, rutrum vitae risus ut, scelerisque laoreet felis. Donec malesuada, dolor quis consequat condimentum, erat nisi condimentum libero, laoreet pulvinar est arcu vitae enim. Nulla malesuada leo sit amet elit molestie, eleifend sodales tellus posuere. Proin vel consequat elit. Curabitur lectus ligula, sollicitudin nec est ut, commodo aliquet velit. Aliquam tempor eros ac nisi consequat posuere. Sed eu mi nibh. Maecenas non dictum turpis. Nunc sed massa vitae lacus luctus lobortis.

Suspendisse rutrum quam a massa tempor, id sollicitudin eros pulvinar. Morbi arcu mi, pellentesque elementum aliquam eu, gravida et nunc. Integer feugiat bibendum risus et sodales. Ut a sollicitudin orci, viverra pulvinar est. Donec ut lacus sit amet libero sodales vehicula. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Suspendisse vel dui nec ipsum lacinia dictum. Maecenas ornare nisi blandit eleifend mollis. Sed est tellus, aliquet convallis purus in, vehicula eleifend eros. Aenean semper odio ut nibh hendrerit commodo eget ac tellus. Integer faucibus ullamcorper nulla, eu consectetur nisl pharetra et. Nulla facilisi. Nullam fringilla consectetur lorem, sed commodo elit blandit at. Fusce ut magna nunc. Nulla facilisi.

Cras fermentum ullamcorper consequat. Donec tempus vel mi vitae consectetur. Maecenas fermentum, urna nec viverra laoreet, ante sem varius est, vitae molestie mi diam vitae ligula. Aliquam aliquet iaculis quam, luctus dignissim mi sagittis at. Nullam luctus, augue ac vulputate ultrices, dolor ipsum finibus diam, quis viverra dui eros vel odio. Donec vitae diam efficitur, volutpat leo eget, sollicitudin nunc. Vestibulum dictum dui eros, in maximus massa accumsan condimentum. Sed nec interdum nunc. Duis arcu dui, egestas vitae sapien ac, auctor suscipit dolor. Praesent eu sapien odio. Aenean ex lectus, fringilla vitae eros vitae, mattis porttitor felis. Praesent turpis ipsum, fermentum eget libero id, ullamcorper consectetur felis. Suspendisse ultricies consectetur dictum.

Duis lobortis iaculis dictum. Aliquam vitae orci at arcu varius ultricies. Quisque et eros in velit sodales luctus. Fusce ornare nisl velit, sit amet faucibus ex gravida eu. Etiam est augue, aliquet quis leo ac, sollicitudin pulvinar leo. In ornare luctus lorem, eget suscipit dui. Fusce at volutpat dolor. Donec vel bibendum sem, ut posuere risus. Proin consectetur, purus quis dignissim hendrerit, sapien quam venenatis velit, ac posuere erat odio ut erat. Vivamus sit amet posuere ex. Donec a consectetur magna. Ut vestibulum mollis felis eget aliquam. Praesent dapibus ullamcorper dui, nec consequat ex tempus et. Proin a auctor dui, nec pellentesque sem. Integer felis mauris, tincidunt a est vel, porta congue dolor.

Suspendisse fringilla ut neque sed ullamcorper. Integer ornare libero a dui faucibus venenatis. Suspendisse in sollicitudin tellus. Interdum et malesuada fames ac ante ipsum primis in faucibus. In sollicitudin ante vitae eros fringilla, sed vulputate eros imperdiet. Vivamus feugiat justo at odio egestas egestas. Cras pharetra velit non lorem ornare, ut faucibus elit finibus. Donec faucibus odio id nulla rhoncus, in volutpat lacus blandit. Phasellus semper justo mattis vulputate pharetra. Nulla in tellus enim.

Suspendisse volutpat, nibh sit amet placerat lobortis, augue lectus efficitur augue, at porttitor ipsum nulla in arcu. Etiam pellentesque consectetur lobortis. Donec rhoncus enim facilisis ligula facilisis auctor. Integer et placerat libero, a fringilla augue. In pulvinar mollis tortor, ac condimentum leo gravida commodo. Ut aliquet elementum dolor eget egestas. Aliquam eu placerat felis. Morbi nec augue cursus, porttitor mi ac, efficitur felis. Maecenas suscipit ornare leo, ut pharetra ante aliquet vehicula. In blandit faucibus massa at fermentum. Vestibulum vulputate vehicula neque ut elementum. Nam hendrerit consectetur congue.

Phasellus rutrum arcu eu ipsum cursus, dictum imperdiet lectus accumsan. Sed nec nulla eu justo vulputate imperdiet. Phasellus semper justo ipsum, ac pretium ex suscipit sit amet. Aenean a viverra nulla. Aliquam erat volutpat. Ut aliquam nisi ornare, tempor ipsum nec, molestie lectus. Vivamus in felis eget leo consectetur tincidunt.

Interdum et malesuada fames ac ante ipsum primis in faucibus. Integer tortor ligula, feugiat ac imperdiet facilisis, cursus a mauris. Nullam bibendum metus augue, a tristique urna pellentesque id. Mauris eu turpis et ipsum mattis ullamcorper vitae ac tellus. Integer sit amet turpis nec mauris porta tempor. Etiam et est molestie, dapibus mauris eget, vehicula sem. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Nunc feugiat felis ac nunc auctor, vitae suscipit sem lobortis. Morbi faucibus metus quis nisl fermentum, et tempus urna porta. Praesent vestibulum tortor ut massa porttitor, sed maximus mi ornare. Nunc pulvinar vitae libero et laoreet. Praesent in erat in nibh rhoncus ullamcorper. Proin et odio erat. Morbi ac bibendum neque. Vestibulum aliquam eleifend ex, id tempus felis lobortis eget.

Suspendisse imperdiet, eros at lacinia pellentesque, velit turpis vestibulum nisl, nec fermentum urna tortor eget nulla. Cras mattis dictum elit vitae vehicula. Mauris ac porta est. Vivamus nulla mauris, posuere at lorem et, lobortis gravida elit. Vestibulum facilisis eleifend maximus. Aenean velit risus, egestas ac ornare sit amet, placerat eget nisl. Mauris non lacus id mauris posuere lobortis. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus.

Duis tempor erat justo, sed consectetur ante dictum ac. Maecenas mollis nunc sed quam porttitor varius. Nunc dapibus velit ac velit vulputate imperdiet. Aenean nisl velit, pulvinar sed iaculis eget, convallis eu purus. Pellentesque aliquam arcu eros, ornare elementum diam lobortis nec. Donec vitae facilisis nunc. Mauris at porta nisl, dapibus volutpat nunc. Aenean risus diam, venenatis sit amet justo eu, rhoncus tincidunt quam. Praesent facilisis accumsan massa, ac placerat felis placerat a. Quisque semper felis sed quam ullamcorper sagittis.

Morbi at ligula at nibh sagittis vulputate non et ante. Nulla eu pulvinar augue, ut venenatis elit. Duis commodo in nisl sit amet pretium. Mauris pellentesque dictum libero, ac tristique dolor ultricies eget. Fusce a metus libero. Nullam dignissim turpis sed turpis placerat, eget facilisis justo sodales. Aliquam erat volutpat. Integer laoreet, odio eu scelerisque suscipit, tellus eros laoreet enim, a laoreet augue urna vitae dolor. Nulla at congue nibh. Phasellus quis lorem orci. Aenean vestibulum felis arcu, ut malesuada sapien laoreet quis. Vestibulum non luctus lacus. Curabitur varius malesuada orci ut molestie. Morbi convallis erat quis velit fermentum, ac facilisis orci varius. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus.

Curabitur faucibus sem sit amet est suscipit, vitae semper enim elementum. Mauris luctus scelerisque enim, vitae ultricies est rutrum quis. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Integer a ex turpis. Nulla a condimentum ante, non eleifend tortor. Praesent lacinia non orci eget lobortis. Ut luctus suscipit porta. Nulla facilisi.

Nunc at elit faucibus, posuere lectus in, semper odio. Aliquam non ullamcorper odio. Maecenas at congue sapien, eget accumsan nisi. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Morbi magna ex, vulputate at congue sit amet, varius pulvinar lectus. Phasellus quis posuere ligula. Mauris ornare quam a tellus eleifend, vel volutpat ex mattis. Pellentesque ullamcorper bibendum lorem a euismod. Nulla finibus a est eu tempus. Mauris porttitor urna non erat sagittis, id tempor augue tristique. Aliquam sem justo, euismod hendrerit tincidunt ac, egestas tincidunt odio. Ut elementum id quam nec imperdiet. Fusce blandit laoreet nulla sit amet ullamcorper. Curabitur mattis mollis mollis. Nullam ut purus sit amet elit dictum molestie. Quisque imperdiet magna in facilisis blandit.

Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Nullam pulvinar tincidunt justo, non elementum leo placerat ut. Mauris eget viverra velit, id eleifend metus. Duis sed risus et ipsum vulputate ullamcorper sit amet a tellus. Integer vestibulum urna vel augue convallis, non pellentesque ligula dictum. Suspendisse dapibus porttitor orci, eu dignissim ante aliquam vestibulum. In maximus, dui quis rhoncus feugiat, magna leo tincidunt arcu, a aliquet libero erat vel mi. Ut consectetur tellus ut nulla hendrerit volutpat. Aliquam malesuada arcu sit amet porttitor maximus. Aliquam erat volutpat. Sed nec ex vel nulla pharetra tincidunt eget id risus. Etiam condimentum tincidunt mattis. Etiam vulputate turpis nec odio ultricies eleifend.

Sed aliquet, sapien eget consequat tempus, elit augue viverra justo, in convallis metus quam at sem. Proin placerat, eros ac auctor eleifend, quam tortor euismod magna, et gravida mi risus sed nisl. Fusce eu quam eros. Curabitur non odio placerat nulla semper semper quis at eros. In nisi nibh, volutpat nec imperdiet eget, blandit ac mi. Sed ac ex a lectus sagittis mattis. In vel purus quis tellus viverra sollicitudin sit amet vitae est.

Nam vel erat velit. Proin quis arcu at erat sollicitudin ullamcorper. Suspendisse fermentum auctor quam, eget consequat nisl porta et. Integer libero neque, sagittis ac scelerisque nec, lobortis vitae dolor. Ut quis bibendum diam. Nunc eu orci non mauris porttitor congue. Donec sit amet viverra ante. Nam at feugiat lorem. In sed velit magna. Aenean eget nulla non mi efficitur tristique sit amet a eros. Vestibulum ut lobortis mauris. Interdum et malesuada fames ac ante ipsum primis in faucibus. Vivamus lacinia aliquet lorem sed hendrerit. Vivamus eleifend ut quam vitae consequat. Curabitur condimentum sollicitudin dolor, a pellentesque eros gravida viverra.

Quisque mattis nisl id orci placerat dictum sed id neque. Etiam ac mi ut libero mattis commodo. Morbi sollicitudin cursus orci, vel ornare eros varius sed. In consectetur ipsum non metus luctus, non vestibulum metus sagittis. Cras convallis pellentesque tempor. Cras sit amet lectus enim. Donec in pulvinar nulla, in maximus odio.

Phasellus sagittis aliquam urna, eget aliquam dolor malesuada sit amet. Pellentesque interdum a nulla et porttitor. Nullam et tempus lorem. Proin ornare scelerisque iaculis. Donec malesuada, enim a commodo lacinia, lectus sapien laoreet mauris, et consequat nibh mauris a tellus. Sed sollicitudin pharetra pharetra. In condimentum scelerisque dui vitae aliquet. Morbi facilisis, dui mattis rhoncus interdum, dui dolor blandit odio, eget sagittis nulla augue in leo. Fusce placerat dolor euismod malesuada mollis.

Mauris ex tellus, placerat sed purus at, facilisis efficitur dolor. Sed ac ex rhoncus, faucibus nisl ac, volutpat odio. Nullam a quam sit amet risus venenatis imperdiet in quis felis. Praesent auctor sollicitudin quam ut vehicula. Nam a tellus ut est facilisis ornare. Praesent ornare in turpis quis consectetur. Ut efficitur, diam a sagittis auctor, velit nisl auctor turpis, vel rhoncus ipsum eros eget velit. Ut sit amet velit vitae lacus auctor bibendum nec ut odio. Nam urna sapien, tristique nec est eget, imperdiet auctor est. Nullam sollicitudin felis vitae est rhoncus porttitor. Duis gravida, enim ac bibendum ultricies, urna justo vulputate augue, id elementum odio felis gravida lorem.

Cras felis tellus, egestas ut varius quis, tempor et felis. Mauris in purus auctor, dictum sapien vitae, tristique dolor. Ut felis nisi, feugiat a efficitur ut, imperdiet sit amet tellus. Quisque ante leo, efficitur quis mi hendrerit, posuere congue ante. "
                            </p>
                        </div>
                        <div class="col-2">
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}
