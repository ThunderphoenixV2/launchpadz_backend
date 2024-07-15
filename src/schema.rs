// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    user,
    users,
);
