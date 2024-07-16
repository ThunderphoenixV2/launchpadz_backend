// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Text,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        name -> Varchar,
        description -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    user,
    users,
);
