use rocket::{serde::json::Json, http::Status, State};
use serde::Serialize;

use crate::{database::{diesel::{get_from_pool, DbPool}, models::encounters::EncounterSelectable}, logic::auth::auth_user};
use diesel::prelude::*;
use crate::database::schema::encounters::dsl::*;
use crate::logic::auth::Token;

#[derive(Serialize)]
pub struct EncounterData{
    title: String,
    description: Option<String>,
    latitude: f32,
    longitude: f32,
    image_urls: Option<String>,
}

#[get("/get", format = "json")]
pub fn get(dbpool: &State<DbPool>, user_token: Token) -> Result<Json<Vec<EncounterSelectable>>, Status> {
    let connection: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>> = &mut get_from_pool(dbpool).unwrap();
    if !auth_user(dbpool, &user_token.value) {
        return Err(Status::Unauthorized)
    }

    Ok(Json(encounters.select(EncounterSelectable::as_select()).load(connection).unwrap()))
}

#[get("/get_radius/<lat>/<long>?<radius>", format = "json")]
pub fn get_radius(dbpool: &State<DbPool>, user_token: Token, lat: f32, long: f32, radius: f32) -> Result<Json<Vec<EncounterSelectable>>, Status> {
    let connection: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>> = &mut get_from_pool(dbpool).unwrap();
    if !auth_user(dbpool, &user_token.value) {
        return Err(Status::Unauthorized)
    }

    Ok(Json(encounters.filter(longitude.gt(long - radius))
                .filter(longitude.lt(long + radius))
                .filter(latitude.gt(lat - radius))
                .filter(longitude.lt(lat + radius))
                .select(EncounterSelectable::as_select()).load(connection).unwrap()))
}