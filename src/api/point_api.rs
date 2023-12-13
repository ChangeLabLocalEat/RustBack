use crate::{models::point_model::Point, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[get("/locations/<longitudeX>/<latitudeY>/<distanceZ>")]
pub fn get_all_points(db: &State<MongoRepo>, longitudeX: &str, latitudeY: &str, distanceZ: &str) -> Result<Json<Vec<Point>>, Status> {
    let points = db.get_points(longitudeX, latitudeY, distanceZ);   
    match points {
        Ok(points) => Ok(Json(points)),
        Err(_) => Err(Status::InternalServerError),
    }
}