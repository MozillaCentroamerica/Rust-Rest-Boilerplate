use rocket::response::content;
use crate::models::structs::Response;
use crate::models::structs::Login;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use serde_json::{json as sjson, Value};
use rocket::request::{self, Request, FromRequest};
use frank_jwt::{Algorithm, encode, decode};
use rocket::Outcome;
use crate::models::structs::User;
use crate::inc::db::{mongo_client};
use mongodb::{Bson, bson, doc};
use bcrypt::{DEFAULT_COST, hash, verify};

#[post("/auth", format = "application/json", data = "<data>")]
pub fn auth(data: Json<Login>) -> JsonValue {
    let mut response = Response {
        status: 200,
        message: "Hola Mundo!".to_owned(),
        data: "".to_owned()
    };

    let coll = mongo_client("users".to_string());

    let username = data.0.username;
    let password = data.0.password;
    let hashed_password = hash(&password, DEFAULT_COST).unwrap();
    let doc_find = doc! {
        "username": &username,
    };
//    let doc = doc! {
//        "username": &username,
//        "password": &hashed_password,
//    };

    // Insert document into 'test.movies' collection
//    coll.insert_one(doc.clone(), None)
//        .ok().expect("Failed to insert document.");

    // Find the document and receive a cursor
    let mut cursor = coll.find(Some(doc_find.clone()), None)
        .ok().expect("Failed to execute find.");

    let item = cursor.next();

    // cursor.next() returns an Option<Result<Document>>
    match item {
        Some(Ok(doc)) => match doc.get("password") {
            Some(&Bson::String(ref title)) => {
                println!(" Password is: {}", title);
                let valid = verify(&password, &title).unwrap();
                println!(" Password is: {}", valid);
                if(valid){
                    response.status = 200;
                    response.message = "Usuario valido".to_string();
                    //HS256
                    let payload = sjson!({
                        "username": username,
                        "id": 0,
                        "is_admin": true
                    });

                    let header = sjson!({});
                    let secret = "akdjfals";
                    let jwt = encode(header, &secret.to_string(), &payload, Algorithm::HS256).unwrap();

                    response.data = jwt;
                }else{
                    response.status = 403;
                    response.message = "Usuario no valido".to_string()
                }

            },
            _ => panic!("Expected title to be a string!"),
        },
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => {
            panic!("Server returned no results!");
            response.status = 403;
            response.message = "Usuario no valido".to_string()
        },
    }
    json!(response)
}

#[get("/me", format = "application/json",)]
pub fn profile(user: User)-> JsonValue  {
    let message: String = format!("El username loggeado es {}",user.username);
    println!("{}",message);
    let mut response = Response {
        status: 200,
        message: message,
        data: "".to_owned()
    };
    response.data = "Holi".to_string();
    json!(response)

}