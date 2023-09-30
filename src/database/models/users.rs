use diesel::{prelude::Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::Users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String
}
