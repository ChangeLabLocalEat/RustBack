use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub Location_coordinates : Vec<String>,
    pub Location_type: String 
}