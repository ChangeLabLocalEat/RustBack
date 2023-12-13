use std::env;
extern crate dotenv;
use dotenv::dotenv;


use mongodb::{
    bson::{extjson::de::Error, doc},
    results::{InsertOneResult },
    sync::{Client, Collection},
};
use crate::models::point_model::Point;
use crate::models::location_model::Location;

pub struct MongoRepo {
    col: Collection<User>,
    col_point: Collection<Point>,
    col_user: Collection<User>
}
use crate::models::{user_model::User, login_model::Login};

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        let col_point: Collection<Point> = db.collection("Point");
        let col_user: Collection<User> = db.collection("User");
        MongoRepo { col,col_point,col_user }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            first_name: new_user.first_name,
            last_name: new_user.last_name,
            email: new_user.email,
            password: new_user.password,
            username: new_user.username
        };
        let user = self
            .col_user
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_points(&self, longitudeX: &str, latitudeY: &str, distanceZ: &str) -> Result<Vec<Point>, Error> {
        
        let longitude_x: f64 = longitudeX.parse().unwrap();
        let latitude_y: f64 = latitudeY.parse().unwrap();
        let distance_z: f64 = distanceZ.parse().unwrap();

        let query = doc! (
            "location": {
                "$near": {
                    "$geometry": {
                        "type": "Point" ,
                        "coordinates": [ longitude_x , latitude_y ]
                    },
                    "$maxDistance": distance_z
                }
            }
        );
        let cursor = self.col_point.find(query, None).unwrap();
        let mut points: Vec<Point> = Vec::new();
        for result in cursor {
            if let Ok(point) = result {
                points.push(point);
            }
        }
        Ok(points)
    }
    
    pub fn get_user_by_username(&self, login : Login) -> Result<User, Error> {
        let user = self
                .col_user
                .find_one(doc! {"username": login.username, "password": login.password}, None)
                .ok()
                .expect("Unknown login");
        Ok(user.unwrap())
    }
}