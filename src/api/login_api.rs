use crate::models::jwt::{Claims, JWT};
use crate::models::network::{NetworkResponse, ResponseBody, Response};
use crate::{models::user_model::User, models::login_model::Login, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};
use chrono::Utc;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{encode, decode, Algorithm, EncodingKey, Header, DecodingKey, Validation};
use std::env;

pub fn login_user(
    db: &State<MongoRepo>,
    login: Json<Login>,
) -> Result<String, NetworkResponse> {

    let login = login.into_inner();
    let data = Login {
        email : login.email.to_owned(),
        password : login.password.to_owned()
    };
    let user = match db.get_user_by_email(data) {
        Ok(user ) => Ok(user),
        Err(err) => Err(NetworkResponse::NotFound(err.to_string())),
    };

    match create_jwt(user.unwrap()) {
        Ok(token) => Ok(token),
        Err(err) => Err(NetworkResponse::BadRequest(err.to_string())),
    }
}

#[post("/login", format="json", data = "<login>")]
pub fn login(database: &State<MongoRepo>, login: Json<Login>) -> Result<String, NetworkResponse> {
    let token = login_user(database, login)?;

    let response = ResponseBody::jwt(token) ;

    Ok(serde_json::to_string(&response).unwrap())
    
}

pub fn create_jwt(user : User) -> Result<String, Error> {

    let secret = env::var("JWT_KEY").expect("lDFZs3CszgUHA2vwBqub6DHdbEik3CAs");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        subject_id: user.id,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {

    let secret = env::var("JWT_KEY").expect("JWT_KEY must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned())
    }
}