// @generated automatically by Diesel CLI.

diesel::table! {
    Users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}
