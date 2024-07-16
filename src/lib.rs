#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Value};

pub mod schema;
pub mod models;
pub mod db;
pub use db::{*, handle_users::handle_users::*};

// error function
#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
