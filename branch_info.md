# frontend_design branch info

This branch is for making better frontend design choices and features. It is also meant for improving frontend readability.

## Branch todo

### General design

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

### Base of exercises

- Base of exercise building
- Base of exercise access setting and sharing.
- Collecting based on source (e. g. book, article).
- Adding problems.
- Merging, splitting, organizing.
- Search.

### Exercise
- Attributes (e. g. source, math subject, status ...)
- Discussion, responses.
- Enable photos loading and editing.

### Learning resources
- Enable students to share their notes.
- List of useful links.
- Educational programs sharing feature.

### Messages and conversations
- Show and hide chat message details.
- Edit messages.
- Display friends at sidebar (when in `chat_with_friends_display()`).
- Filter chat messages in a conversation based on date.
- Emojis.
- User responses.
- Extract message builder to another file.

### Friends
- Enable people search.
- Enable friend adding mechanism.
- Block people.

### Groups
- Enable group search.
- Enable join group mechanism.
- List of groups.

### User Profile
- User profile picture.
- User profile page.

### Settings
- Language settings - text in different languages.

### Events and event lists
- Make some event cathegory - on a group and public level.

### Mathnet notifications
- Enable notification receiving (mechanism similar to conversation).

### Writing styles
- Enable writing code.
- Enable writing latex

### Project
- Add a page for projects




