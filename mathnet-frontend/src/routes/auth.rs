use rocket::{info};
use rocket::form::{Form, FromForm};
use rocket::http::{CookieJar};
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

#[get("/login")]
pub fn get_login() -> Template {
  Template::render("login", context! {
    title: "Login",
  })
}

#[derive(FromForm)]
pub struct Login {
  username: String,
  password: String,
}

#[post("/login", data = "<data>")]
pub fn post_login(data: Form<Login>, cookies: &CookieJar<'_>) -> Redirect {
  /*
   * 1. Get data posted to login endpoint (use form in get request)
   * 2. Make API call to backend
   * 3. Make relevant things so user is logged in (porbably cookies?)
   */
  info!("{} {}", data.username, data.password);

  // TODO: Check data

  // Save user information for session
  cookies.add_private(("user_id", "1"));

  Redirect::to(uri!("/"))
}

#[get("/logout")]
pub fn get_logout(cookies: &CookieJar<'_>) -> Redirect {
  // Clear user information from session
  cookies.remove_private("user_id");

  Redirect::to(uri!("/"))
}

#[get("/register")]
pub fn get_register() -> Template {
  Template::render("register", context! {
    title: "Register",
  })
}

#[derive(FromForm)]
pub struct Register {
  username: String,
  email: String,
  password: String,
  password_check: String,
}

#[post("/register", data = "<data>")]
pub fn post_register(data: Form<Register>) -> Redirect {
  /*
   * 1. Get data posted to register endpoint (use form in get request)
   * 2. Make API call to backend
   * 3. Make relevant things so user is logged in (porbably cookies?)
   */
  info!("{} {} {} {}", data.username, data.email, data.password, data.password_check);

  // TODO: Check data

  Redirect::to(uri!("/"))
}
