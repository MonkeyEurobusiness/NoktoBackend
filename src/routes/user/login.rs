use rocket::{serde::{json::Json, Deserialize, Serialize}, http::Status, State};

use crate::{database::{diesel::{get_from_pool, DbPool}, models::users::{User, Session, UserInsertable}}, logic::auth::auth_user};
use diesel::{prelude::*, insert_into, update};
use crate::database::schema::users::dsl::*;
use crate::database::schema::sessions::dsl::*;
use crate::logic::auth::Token;


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

#[post("/logout", format = "json")]
pub fn logout(dbpool: &State<DbPool>, user_token: Token) -> Status {
    let connection: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>> = &mut get_from_pool(dbpool).unwrap();
    if !auth_user(dbpool, &user_token.value) {
        return Status::Unauthorized
    }

    let results: Vec<String> = sessions.filter(token.eq_all(&user_token.value))
        .select(token)
        .load(connection).unwrap();
    if results.len() != 1 {
        return Status::InternalServerError;
    }

    let _ = update(sessions.filter(token.eq_all(&user_token.value))).set(logout_date.eq(chrono::Utc::now().naive_utc()))
    .execute(connection);
    Status::Ok
}

#[derive(Deserialize)]
pub struct RegisterData{
    username: String,
    password: String
}

#[post("/register", format = "json", data = "<data>")]
pub fn register(dbpool: &State<DbPool>, data: Json<RegisterData>) -> Status {
    let connection: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>> = &mut get_from_pool(dbpool).unwrap();

    let results = users.filter(username.eq_all(&data.username))
        .select(User::as_select())
        .load(connection).unwrap();

    if !results.is_empty() {
        return Status::Forbidden;
    }

    let new_user = UserInsertable { 
        username: data.username.clone(), 
        password: data.password.clone(),
    };

    let _ = insert_into(users)
        .values(new_user)
        .execute(connection);
    
    Status::Ok
}