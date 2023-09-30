// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (token) {
        user_id -> Integer,
        token -> Text,
        login_date -> Timestamp,
        logout_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
