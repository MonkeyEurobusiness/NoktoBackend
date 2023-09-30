use rocket::{serde::{json::Json, Deserialize, Serialize}, http::Status, State};

use crate::database::{diesel::{get_from_pool, DbPool}, models::users::{User, Session}};
use diesel::{prelude::*, insert_into, update};
use crate::database::schema::users::dsl::*;
use crate::database::schema::sessions::dsl::*;


use rand::Rng;
use rand::distributions::Alphanumeric;
use rand::rngs::OsRng;

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

    let user = results.get(0).unwrap();

    let rnd: String = OsRng
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    let new_session = Session {
        user_id: user.id,
        token: rnd.clone(),
        logout_date: None
    };
    let _ = insert_into(sessions)
        .values(new_session)
        .execute(connection);

    Ok(Json(LoginResponse {
        token: rnd
    }))
}

#[derive(Deserialize)]
pub struct LogoutData {
    token: String,
}

#[post("/logout", format = "json", data = "<data>")]
pub fn logout(dbpool: &State<DbPool>, data: Json<LogoutData>) -> Status {
    let connection: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>> = &mut get_from_pool(dbpool).unwrap();

    let results: Vec<String> = sessions.filter(token.eq_all(&data.token))
        .select(token)
        .load(connection).unwrap();
    if results.len() != 1 {
        return Status::InternalServerError;
    }

    let _ = update(sessions.filter(token.eq_all(&data.token))).set(logout_date.eq(chrono::Utc::now().naive_utc()))
    .execute(connection);
    Status::Ok
}