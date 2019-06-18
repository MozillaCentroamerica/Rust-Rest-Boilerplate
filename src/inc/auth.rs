
use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

pub extern crate crypto;
pub extern crate jwt;
pub extern crate rustc_serialize;
use serde_json::{json as sjson, Value};

use frank_jwt::{Algorithm, encode, decode};
use rocket_contrib::json::JsonValue;


pub struct ApiKey(pub String);

pub fn read_token(key: &str) {
    let secret = "akdjfals";
    let mut header = sjson!({});
    let mut payload = sjson!({
        "username": "",
        "password": "",
        "id": 0,
        "is_admin": true
    });
    let (header, payload) = decode(&key, &secret.to_string(), Algorithm::HS256).unwrap();
    println!("{}", payload.username);
}
impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
//        match read_token(keys[0]) {
//            Ok(claim) => Outcome::Success(ApiKey(claim)),
//            Err(_) => Outcome::Forward(())
//        }
        Outcome::Success(ApiKey("".to_owned()))
    }
}