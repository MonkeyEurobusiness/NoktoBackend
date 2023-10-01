use diesel::SqliteConnection;
use diesel::prelude::*;
use crate::database::diesel::{get_from_pool, DbPool};
use crate::database::models::users::SessionAll;
use crate::database::models::users::User;
use crate::database::schema::users::dsl::*;
use crate::database::schema::sessions::dsl::*;

use super::auth::Token;

pub fn get_user(dbpool: &DbPool, user_token: Token) -> Option<User> {
    let connection: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>> = &mut get_from_pool(dbpool).unwrap();

    let results = sessions.filter(token.eq_all(&user_token.value))
        .select(SessionAll::as_select())
        .load(connection).unwrap();
    if results.len() != 1 {
        return None
    }

    let found_id = results[0].user_id;

    let mut results = users.filter(id.eq_all(found_id))
        .select(User::as_select())
        .load(connection).unwrap();
    if results.len() != 1 {
        return None;
    }
    
    Some(results.remove(0))
}