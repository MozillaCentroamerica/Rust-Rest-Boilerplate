use serde::{Deserialize, Serialize};
// A trait that the Validate derive will impl
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: u32,
    pub message: String,
    pub data: String
}


#[derive(Serialize, Deserialize,Validate)]
pub struct Login {
    #[validate(length(min = "4"))]
    pub username: String,
    #[validate(length(min = "4"))]
    pub password: String
}
