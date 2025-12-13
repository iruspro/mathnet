# Minimal Goal
The minimal goal to be achieved is that two users can run the program at the same time on different machines and 
have real-time conversation that gets stored and retreived to and from some database.

# Design and features
## General design

~~Improve left sidebar and top navbar design.~~

Simplify and organize frontend code:
- ~~Simplify page enums!~~

- ~~Page reorganization.~~

- ~~Page and view routing need to be updated to match simplified enums in `list_of_pages.rs`.~~

- ~~Add content functions for each logged in and logged out page (to get rid of unneccessary left sidebar and top navbar in each `view` function). Rename `view` functions to `content` functions.~~

- ~~Rewrite every page so that they will match the template idea.~~

- ~~Rewrite `register.rs` so that only content will be present.~~

- ~~Rewrite `displaying_conversation.rs` so that only content will be present.~~

- Comment on code: explain its purpose.

- Merge `frontend_features` and `logics` directories.

- ~~Declare project's naming convention.~~

- Think off how to avoid code duplication.

- Tidy `use` statements.

- Reduce public code where possible.

- ~~Rename `logged_in_pages` to `signed_in_pages` and `logged_out_pages` to `signed_out_pages` (or vice versa - the important thing is to have consistent naming across the project - I decided for 'sign' version).~~

~~Add licence info at the bottom of every page.~~

Add tests. No logic code has been been tested yet.

Make sure code is consistent across mathnet project.

## Base of exercises

- Base of exercise building
- Base of exercise access setting and sharing.
- Collecting based on source (e. g. book, article).
- Adding problems.
- Merging, splitting, organizing.
- Search.

## Exercise
- Attributes (e. g. source, math subject, status ...)
- Discussion, responses.
- Enable photos loading and editing.

## Learning resources
- Enable students to share their notes.
- List of useful links.
- Educational programs sharing feature.

## Messages and conversations
- Show and hide chat message details.
- Edit messages.
- Display friends at sidebar (when in `chat_with_friends_display()`).
- Filter chat messages in a conversation based on date.
- Emojis.
- User responses.
- ~~Extract message builder to another file.~~

## Friends
- ~~Enable people search.~~
- ~~Enable friend search.~~
- ~~Enable friend adding mechanism.~~
- ~~Remove friend.~~
- ~~Block friend.~~
- Block people.
- ~~List of received and sent friend requests.~~

## Groups
- Enable group search.
- Enable join group mechanism.
- List of groups.

## User Profile
- User profile picture.
- User profile page.

## Settings
- Language settings - text in different languages.

## Events and event lists
- Make some event cathegory - on a group and public level.

## Mathnet notifications
- Enable notification receiving (mechanism similar to conversation).

## Writing styles
- Enable writing code.
- Enable writing latex

## Project
- Add a page for projects: similar to groups, but for projects.

## Other
- Write `Search` trait which would include functionalities for searching among
various cathegories.

---
Important note to these goals is that they are **design parts** of available 
functionalitites, i. e. they won't actually work untill communication with 
the backend
is created. After completing these aims functions that send and receive to 
and from the backend will still need to be written. 

# Database
## Learn
- async programming in rust
- sqlx crate
- tokio crate
- sql
- postgreSQL
I learn only the basics.

## Connection/communication
- Sign in
- Register
- Add friend
- Send message
- Store messages
- Get conversation from database

# Reconsidering crate choices
- If things didn't go as easy as expected, switching to other crates (like `rocket`) would have to be considered. The catch is that switching to any 
other crate would demand a lot of code rewriting and time. Therefore the 
best option so far is to make something with `sqlx`.

# Bugs
When navigating between pages in browser, misterious
"Node list must have already unrolled when creating an element" error is raised by sauron. I suspect it has something to do with left sidebar and top navbar, but similar mechanism has worked in my other projects.