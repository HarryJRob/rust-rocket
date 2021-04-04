#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    Template::render("index", ())
}

#[catch(404)]
fn not_found() -> Template {
    Template::render("404", ())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .register(catchers![not_found])
        .attach(Template::fairing())
        .launch();
}