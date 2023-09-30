use rocket::{serde::{json::Json, Deserialize, Serialize}, http::Status, State};

use crate::database::{diesel::{get_from_pool, DbPool}, models::users::User};
use diesel::prelude::*;
use crate::database::schema::users::dsl::*;

#[derive(Deserialize)]
pub struct LoginData{
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct LoginResponse{
    token: String
}

#[post("/login", format = "json", data = "<data>")]
pub fn login(dbpool: &State<DbPool>, data: Json<LoginData>) -> Result<Json<LoginResponse>, Status> {
    let connection: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>> = &mut get_from_pool(dbpool).unwrap();

    let results = users.filter(username.eq_all(&data.username))
        .filter(password.eq_all(&data.password))
        .select(User::as_select())
        .load(connection).unwrap();
    if results.len() != 1 {
        return Err(Status::Unauthorized);
    }
    Ok(Json(LoginResponse {
        token: data.username.clone()
    }))
}