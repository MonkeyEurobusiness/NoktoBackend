use database::diesel::establish_pooled_connection;
use routes::user;

mod routes;
mod database;
mod logic;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .mount("/user", routes![user::login::login, user::login::logout, user::login::register])
    .manage(establish_pooled_connection())
}