use serde::{Deserialize, Serialize};
use rocket_contrib::json::JsonValue;
use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use serde_json::{json, Value};
use frank_jwt::{Algorithm, encode, decode};

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


#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub is_admin: bool,
    pub id: i32
}

/*
FormRequest implement for struct User so we can use it as request param
*/
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        let secret = "akdjfals";
        let mut header = json!({});
        let mut payload = json!({
        "username": "",
        "password": "",
        "id": 0,
        "is_admin": true
    });
        let mut bearer_splited = keys[0].split(" ");
        let mut jwt= "";
        for s in bearer_splited {
            jwt=s; // the last one is the jwt
        }
        let decoded_data = decode(&jwt, &secret.to_string(), Algorithm::HS256);
        if decoded_data.is_ok() {
            let (header, payload) = decoded_data.unwrap();
            println!("User logged in : {}", payload["username"]);

            let mut user: User = User{
                username: payload["username"].to_string(),
                password: payload["password"].to_string(),
                is_admin: false,
                id: 0
            };

            Outcome::Success(user)
        } else {
            println!("Invalid user");
            return Outcome::Forward(());
        }

    }
}