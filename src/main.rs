#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[get("/")]
fn home() -> String {
    format!("/")
}
fn main() {
    rocket::ignite().mount("/", routes![home]).launch();
}
