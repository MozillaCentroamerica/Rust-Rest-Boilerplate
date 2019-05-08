use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: u32,
    pub message: String,
    pub data: String
}


#[derive(Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String
}
