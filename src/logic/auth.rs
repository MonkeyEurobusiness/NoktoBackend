use std::convert::Infallible;

use diesel::SqliteConnection;
use diesel::{prelude::*, insert_into, update};
use rocket::Request;
use rocket::http::Status;
// use rocket::http::hyper::{request};
use rocket::request::{FromRequest, Outcome};
use crate::database::models::users::SessionAll;
use crate::database::{diesel::{get_from_pool, DbPool}, models::users::{User, Session}};
use crate::database::schema::users::dsl::*;
use crate::database::schema::sessions::dsl::*;

#[derive(Debug)]
pub struct Token {
    pub value: String
}

#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = TokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid(key: &str) -> bool {
            // key == "valid_api_key"
            //TODO
            true
        }

        match req.headers().get_one("Token") {
            None => Outcome::Failure((Status::BadRequest, TokenError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(Token{value: key.to_string()}),
            Some(_) => Outcome::Failure((Status::BadRequest, TokenError::Invalid)),
        }
    }
}

pub fn auth_user(dbpool: &DbPool, user_token: &String) -> bool {
    let connection: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>> = &mut get_from_pool(dbpool).unwrap();

    let results: Vec<String> = sessions.filter(token.eq_all(user_token))
        .filter(logout_date.is_null())
        .select(token) //aaaaaaaaaaa TODO
        .load(connection).unwrap();
    if results.len() == 1 {
        return true;
    }
    false
}