// @generated automatically by Diesel CLI.

diesel::table! {
    encounters (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        is_dangerous -> Bool,
        is_abused -> Bool,
        latitude -> Float,
        longitude -> Float,
        image_urls -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

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

diesel::joinable!(encounters -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    encounters,
    sessions,
    users,
);
