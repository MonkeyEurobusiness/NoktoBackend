use database::diesel::establish_pooled_connection;
use routes::user;

mod routes;
mod database;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .mount("/user", routes![user::login::login])
    .manage(establish_pooled_connection())
}