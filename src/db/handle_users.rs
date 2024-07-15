pub mod handle_users {

// ================ //

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))

}

// ================ //

// create user
use crate::models::{ User, NewUser};
pub fn create_user(name: &str, email: &str) -> User {
    use crate::schema::users;

    let connection = &mut establish_connection();

    let new_user = NewUser { name, email };
    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error saving new user")
}

// get user
pub fn get_user(user_id: i32) -> User{
    use crate::schema::users::dsl::*;

let connection = &mut establish_connection();

let user_profile = users
    .find(user_id)
    .select(User::as_select())
    .first(connection)
    .expect("Error fetching user");
user_profile
}

// update user

// delete user
pub fn delete_user(user_id: i32) {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let pattern = format!("%{user_id}%");

    diesel::delete(users.filter(email.like(pattern)))
        .execute(connection)
        .expect("Error deleting user");
}
}