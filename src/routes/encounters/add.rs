use rocket::{serde::{json::Json, Deserialize}, http::Status, State};

use crate::{database::{diesel::{get_from_pool, DbPool}, models::encounters::EncounterInsertable}, logic::{auth::auth_user, user::get_user}};
use diesel::{prelude::*, insert_into};
use crate::database::schema::encounters::dsl::*;
use crate::logic::auth::Token;

#[derive(Deserialize)]
pub struct AddData{
    title: String,
    description: Option<String>,
    is_dangerous: bool,
    is_abused: bool,
    latitude: f32,
    longitude: f32,
    image_urls: Option<String>,
}

#[put("/add", format = "json", data = "<data>")]
pub fn add(dbpool: &State<DbPool>, data: Json<AddData>, user_token: Token) -> Status {
    let connection: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>> = &mut get_from_pool(dbpool).unwrap();
    if !auth_user(dbpool, &user_token.value) {
        return Status::Unauthorized
    }
    
    let Some(user) = get_user(dbpool, user_token) else {return Status::ImATeapot};

    let new_encounter = EncounterInsertable {
        user_id: user.id,
        title: data.title.clone(),
        description: data.description.clone(),
        is_dangerous: data.is_dangerous,
        is_abused: data.is_abused,
        latitude: data.latitude,
        longitude: data.longitude,
        image_urls: data.image_urls.clone(),
    };

    let _ = insert_into(encounters)
        .values(new_encounter)
        .execute(connection);
    
    Status::Ok
}