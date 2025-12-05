# frontend_design branch info

This branch is for making better frontend design choices and features. It is also meant for improving frontend readability.

## Branch todo

~~Improve left sidebar and top navbar design.~~

Simplify frontend code
- ~~Simplify page enums!~~

- Page and view routing need to be updated to match simplified enums in 
`list_of_pages.rs`

- Add content functions for each logged in and logged out page (to get rid of unneccessary left sidebar and top navbar in each `view` function). Rename `view` functions to `content`
functions.
-Rewrite every page so that they will match the template idea.


Rename `logged_in_pages` to `signed_in_pages` and `logged_out_pages` to `signed_out_pages` (or vice versa - the important thing is to have consistent naming across the project).

Add licence info at the bottom of every page.

Features
- Show and hide chat message details.
- Filter chat messages in a converstion based on date.
- Latex rendering.
- User profile picture.
- Searching for friends, add friends, send chat messages.
- Edit chat message.
