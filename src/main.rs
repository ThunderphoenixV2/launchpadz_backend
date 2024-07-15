#[macro_use]
extern crate rocket;
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};
use std::str::FromStr;
use launchpadz_backend::*;

// ===================================================================== //

#[launch]
fn rocket() -> _ {

    // cors configurations
    let allowed_methods: AllowedMethods = ["GET", "POST", "DELETE"]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();

    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);
    
    let cors = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .allowed_methods(allowed_methods)
        .allowed_headers(AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
        ]))
        .allow_credentials(true)
        .to_cors()
        .expect("Error creating CORS fairing");
    
    //rocket start in 3...2...1
    rocket::build()
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            port: 8000,
            ..rocket::Config::default()
        })
        .mount("/", routes![return_data, get_data])
        .register("/", catchers![not_found])
        .manage(MessageList::new(vec![]))                                  //remove later, aswell as the mod import
        .attach(cors)
}
