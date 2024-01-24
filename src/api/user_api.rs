use crate::{models::{jwt::JWT, network::NetworkResponse, user_model::User}, repository::mongodb_repo::MongoRepo};
use bson::oid::ObjectId;
use mongodb::results::{InsertOneResult, UpdateResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/register", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        firstName: new_user.firstName.to_owned(),
        lastName: new_user.lastName.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned()
    };
    let user_detail = db.create_user(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<id>")]
    pub fn get_user(db: &State<MongoRepo>, id: String) -> Result<Json<User>, Status> {
        if id.is_empty() {
            return Err(Status::BadRequest);
        };
        let user_detail = db.get_user(&id);
        match user_detail {
            Ok(user) => Ok(Json(user)),
            Err(_) => Err(Status::InternalServerError),
        }
    }

#[put("/user/<id>", data = "<current_user>")]
pub fn update_user(
    db: &State<MongoRepo>,
    id: String,
    current_user: Json<User>,
) -> Result<Json<User>, Status> {
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        firstName: current_user.firstName.to_owned(),
        lastName: current_user.lastName.to_owned(),
        email: current_user.email.to_owned(),
        password: current_user.password.to_owned()
    };
    let update_result = db.update_user(&id, data);
    print!("update_result {:?}", update_result);
    match update_result {
        Ok(updated) => {
            if updated.matched_count == 1 {
                let updated_user_info = db.get_user(&id);
                return match updated_user_info {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

// Pour activer l'authentification, il faut ajouter le code suivant au début de la fonction, en plus de "key: Result<JWT, NetworkResponse>"
// en paramètre de la fonction
// let key = match key {
//     Ok(key) => key,
//     Err(_) => return Err(Status::InternalServerError),
// };