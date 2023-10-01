use chrono::NaiveDateTime;
use diesel::{prelude::{Insertable, Queryable}, Selectable};
use serde::Serialize;

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::encounters)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EncounterInsertable {
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub latitude: f32,
    pub longitude: f32,
    pub image_urls: Option<String>,
}

#[derive(Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::encounters)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EncounterSelectable {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub latitude: f32,
    pub longitude: f32,
    pub image_urls: Option<String>,
    pub created_at: NaiveDateTime,
}