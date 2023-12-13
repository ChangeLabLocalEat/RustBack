use mongodb::bson::oid::ObjectId;
use crate::api::login_api::decode_jwt;
use rocket::{serde::{Deserialize, Serialize}, http::Status};
use crate::models::network::{NetworkResponse, ResponseBody};
use rocket::request::{Outcome, Request, FromRequest};
use jsonwebtoken::errors::Error;
use jsonwebtoken::errors::ErrorKind;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: Option<ObjectId>,
    pub exp: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => {
                let response = ResponseBody::Message(
                        String::from("Error validating JWT token - No token provided")
                    );

                Outcome::Error((
                    Status::Unauthorized, 
                    NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                ))
            },
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT {claims}),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let response = ResponseBody::Message(
                                format!("Error validating JWT token - Expired Token")
                            );

                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                        )) 
                    },
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        let response = ResponseBody::Message(
                                format!("Error validating JWT token - Invalid Token")
                            );

                        Outcome::Error((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                        )) 
                    },
                    _ => {
                        let response = ResponseBody::Message(
                                format!("Error validating JWT token - {}", err)
                            );

                        Outcome::Error((
                            Status::Unauthorized, 
                            NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                        )) 
                    }
                }
            },
        }
    }
}