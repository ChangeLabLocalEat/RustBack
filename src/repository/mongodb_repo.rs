use std::env;
extern crate dotenv;
use bson::oid::ObjectId;
use dotenv::dotenv;


use mongodb::{
    bson::{extjson::de::Error, doc},
    results::{InsertOneResult, UpdateResult },
    sync::{Client, Collection},
};
use crate::models::point_model::Point;


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
            firstName: new_user.firstName,
            lastName: new_user.lastName,
            email: new_user.email,
            password: new_user.password
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

        // Pue la merde à changer
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
        let cursor = self.col_point.find(None, None).unwrap();
        let mut points: Vec<Point> = Vec::new();
        for result in cursor {
            if let Ok(point) = result {
                points.push(point);
            }
        }
        Ok(points)
    }
    
    pub fn get_user_by_email(&self, login : Login) -> Result<User, Error> {
        let user = self
                .col_user
                .find_one(doc! {"email": login.email, "password": login.password}, None)
                .ok()
                .expect("Unknown login");
        Ok(user.unwrap())
    }

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }


    pub fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "firstName": new_user.firstName,
                    "lastName": new_user.lastName,
                    "password": new_user.password,
                    "email": new_user.email,
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }
}