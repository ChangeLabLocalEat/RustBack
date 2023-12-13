use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String
}