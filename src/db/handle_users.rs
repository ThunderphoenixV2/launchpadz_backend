pub mod handle_users {

// ================ //

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    println!("trying to establish connection"); // debug
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    println!("successfully established connections"); // debug
    connection
}

// ================ //

// create user
use crate::models::{ User, NewUser};
use rocket::serde::json::{Value, json};
pub fn create_user(id: &i32, name: &str, description: &str) -> Value {
    println!("trying to create user");  // debug
    use crate::schema::users;

    let connection = &mut establish_connection();

    let new_user = NewUser { id, name, description };
    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error saving new user");
    println!("successfully created user");  // debug
    json!({ "status": "successfully created new user"})
    // return confirmation instead of user (?)
}

// get user
pub fn get_user(user_id: i32) -> User {
    println!("trying to fetch user"); // debug
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let user_profile = users
        .find(user_id)
        .select(User::as_select())
        .first(connection)
        .expect("Error fetching user");
    println!("successfully fetched user");  // debug
    user_profile
}

/*
// delete user
pub fn delete_user(user_id: i32) {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let pattern = format!("%{user_id}%");

    diesel::delete(users.filter(id.eq(pattern)))
        .execute(connection)
        .expect("Error deleting user");
    // return confirmation
}
*/
}