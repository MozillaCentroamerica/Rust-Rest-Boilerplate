use rocket::response::content;
use crate::models::structs::Response;
use crate::models::structs::Login;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use serde_json::json as sjson;

use frank_jwt::{Algorithm, encode, decode};


#[post("/auth", format = "application/json", data = "<data>")]
pub fn auth(data: Json<Login>) -> JsonValue {
    let mut response = Response {
        status: 200,
        message: "Hola Mundo!".to_owned(),
        data: "".to_owned()
    };


    //HS256
    let mut payload = sjson!({
    "username": data.0.username,
    "password": data.0.password,
    "id": 0,
    "is_admin": true
});

    let mut header = sjson!({});
    let secret = "akdjfals";
    let jwt = encode(header, &secret.to_string(), &payload, Algorithm::HS256).unwrap();


    response.data = jwt;
    json!(response)
}