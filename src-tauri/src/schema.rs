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

diesel::table! {
    video (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        removed -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(license_key, user_table, video,);
