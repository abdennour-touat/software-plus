// @generated automatically by Diesel CLI.

diesel::table! {
    license_key (id) {
        id -> Integer,
        hash -> Text,
    }
}

diesel::table! {
    user_table (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    license_key,
    user_table,
);
