use rocket::{info};
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

#[get("/about")]
pub fn get_about() -> Template {
  Template::render("about", context! {
    title: "About",
  })
}

#[get("/docs")]
pub fn get_docs() -> Template {
  Template::render("docs", context! {
    title: "Docs",
  })
}

#[get("/privacy")]
pub fn get_privacy() -> Template {
  Template::render("privacy", context! {
    title: "Privacy",
  })
}
