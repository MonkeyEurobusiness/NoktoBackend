use diesel::prelude::Insertable;

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