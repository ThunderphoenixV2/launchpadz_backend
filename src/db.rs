pub mod handle_users;
use crate::db::handle_users::handle_users::*;

use rocket::futures::lock::Mutex;
use rocket::serde::{
    json::{json, Json, Value},
    Deserialize, Serialize,
};
use rocket::{get, post, State};

// structs
#[derive(Serialize, Deserialize)]
pub struct RequestData {
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseData {
    reply: String,
}

// types
pub type MessageList = Mutex<Vec<String>>;                                          // makeshift DB
pub type Messages<'r> = &'r State<MessageList>;

// get functions
#[get("/api/data")]
pub async fn return_data(list: Messages<'_>) -> Json<ResponseData> {
    let mut list = list.lock().await;                  // change to DB later
    let reply:String;
    match list.pop() {
        Some(n) => reply = n,
        None => reply = String::from("error")
    }
    Json(ResponseData {
        reply: format!("Received: {reply}")
    })
}

// post functions
#[post("/api/data", format = "json", data = "<data>")]
pub async fn get_data(data: Json<RequestData>, list: Messages<'_>) -> Value {
    let mut list = list.lock().await;                  // change to DB later
    list.push(data.message.clone());
    println!("{list:#?}");                                                      // debug
    json!({ "status": "hi, ja hat geklappt"})
}
