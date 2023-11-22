use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use super::location_model::Location;

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title : String,
    pub pointtype : String,
    pub position: Location
}