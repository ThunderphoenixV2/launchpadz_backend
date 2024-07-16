pub mod handle_users;
use rocket::serde::{
    json::{Json, Value},
    Deserialize, Serialize,
};
use rocket::post;
use crate::{create_user, get_user};

// structs
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserData {
    id: i32,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusData {
    status: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserData {
    id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ReturnUserData {
    id: i32,
    name: String,
    description: String,
}

// get functions
#[post("/api/data/get", format = "json", data = "<data>")]
pub async fn return_user_data(data: Json<GetUserData>) -> Json<ReturnUserData> {
    let user = get_user(data.id);
    println!("{user:#?}");
    Json(ReturnUserData {
        id: user.id,
        name: user.name,
        description: user.description
    })
}

// post functions
#[post("/api/data/create", format = "json", data = "<data>")]
pub async fn create_user_data(data: Json<CreateUserData>) -> Json<StatusData> {
    println!("{data:#?}"); // debug
    Json(StatusData {
        status: create_user(&data.id, &data.name, &data.description)
    })
}
