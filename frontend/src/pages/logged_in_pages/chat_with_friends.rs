use sauron::prelude::*;
use crate::messages::{Msg, GoToPage, SwitchToPageSigned};
use crate::app::App;
use crate::structs::friends_at_sidebar::{show_friends_at_sidebar};

pub fn view(data_provided : &App) -> Node<Msg> {
    node! {
        <main>
            // Left Sidebar (Desktop)
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
                        <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToDocsPage))}>"Docs"</a>
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
                        <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToNotifications))}>"Notifications"</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link text-white" on_click=|_|{Msg::SetPage(GoToPage::GoToPageSigned(SwitchToPageSigned::GoToLogOut))}>"Log out"</a>
                    </li>
                </ul>
            </div>

            // Right Sidebar
            <div class="right-sidebar d-none d-md-block text-white">
                <h4>"Right Sidebar"</h4>
                <ul class="nav flex-column">
                <div class="search-container">
        <input type="text" class="form-control search-input" placeholder="Find a person" on_input=|input|{Msg::SearchFriend(input.value())}></input>
        <i class="fas fa-search search-icon"></i>
      </div>
                    show_friends_at_sidebar();
                </ul>
            </div>

            // Main Content
            <div class="content">
                <div class="container-fluid">
                    <h1 class="text-center">"Find friends"</h1>
                    <p class="basicparagraph text-start">
                        "View other people's profiles or chat with somebody.

                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras luctus consectetur placerat. Donec non pretium sapien. Donec ac placerat ex. Aenean tempus massa nulla, nec ullamcorper leo tempus eget. Vestibulum at lectus ut libero ullamcorper consectetur. Vestibulum auctor urna venenatis libero eleifend, in sodales odio dictum. Curabitur auctor, massa eget ultricies efficitur, justo nulla porta purus, in laoreet quam turpis ut risus. Aliquam nunc nibh, placerat eget bibendum varius, porta eget tellus. Phasellus in lacinia augue, sed consequat ex. Curabitur laoreet mi nec eros tristique, ac tristique mauris tempus."
                    </p>
                </div>
            </div>
        </main>
    }
}
