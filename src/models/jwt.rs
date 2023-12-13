use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: Option<ObjectId>,
    pub exp: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}
