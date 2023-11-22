//Ajout de modules
mod api; 
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};
//add imports below
use api::user_api::create_user;
use repository::mongodb_repo::MongoRepo;

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust and mongoDB")))
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![hello])
        .mount("/", routes![create_user])
        .mount("/", routes![api::point_api::get_all_points])
}
