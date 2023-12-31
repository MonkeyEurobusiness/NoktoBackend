use diesel::{prelude::{Queryable, Insertable}, Selectable};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserInsertable {
    pub username: String,
    pub password: String
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::sessions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Session {
    pub user_id: i32,
    pub token: String,
    pub logout_date: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::sessions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SessionAll {
    pub user_id: i32,
    pub token: String,
    pub login_date: NaiveDateTime,
    pub logout_date: Option<NaiveDateTime>,
}