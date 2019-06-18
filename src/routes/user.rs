use rocket::response::content;
use crate::models::structs::Response;
use crate::models::structs::Login;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use serde_json::json as sjson;
use rocket::request::{self, Request, FromRequest};
use frank_jwt::{Algorithm, encode, decode};
use rocket::Outcome;

use crate::inc::auth::ApiKey;


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

#[get("/me")]
pub fn me(_key: ApiKey)-> JsonValue  {
    let mut response = Response {
        status: 200,
        message: "Hola Mundo!".to_owned(),
        data: "".to_owned()
    };
    response.data = "Holi".to_string();
    json!(response)

}

//#[get("/me", format = "application/json")]
//impl<'a, 'r> rocket::request::Request<'a, 'r> for Response {
//    type Error = ();
//
//    pub fn me(request: &'a rocket::Request<'r>,) -> JsonValue {
//        let mut response = Response {
//            status: 200,
//            message: "Hola Mundo!".to_owned(),
//            data: "".to_owned()
//        };
//
//        let header_map = request.headers();
//        response.data = header_map;
//
////    response.data = jwt;
//        json!(response)
//    }
//}