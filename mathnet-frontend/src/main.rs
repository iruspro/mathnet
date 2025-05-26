#[macro_use] extern crate rocket;
use rocket::Build;
use rocket::Rocket;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

mod routes;
use routes::auth::{get_login, post_login, get_logout, get_register, post_register};
use routes::other::{get_about, get_docs, get_privacy};

#[get("/")]
fn index() -> Template {
  Template::render("index", context! {
    title: "Hello", // Title for each page
    name: "Name",
    items: vec!["One", "Two", "Three"],
  })
}

#[catch(404)]
fn not_found() -> Template {
  Template::render("error_not_found", context! {
    title: "Not found",
  })
}

#[launch]
fn rocket() -> Rocket<Build> {
  rocket::build()
    // Mount all routes
    .mount("/", routes![
        index,
        get_login,
        post_login,
        get_logout,
        get_register,
        post_register,
        get_about,
        get_docs,
        get_privacy,
      ])
    // Serve static content from folder static (CSS, JS...)
    .mount("/public", FileServer::from(relative!("static")))
    // Catch erros
    .register("/", catchers![not_found])
    .attach(Template::fairing())
}
