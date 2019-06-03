use rocket::response::content;
use crate::models::structs::Response;

#[get("/")]
pub fn hola() -> content::Json<String> {
    let mut response = Response {
        status: 200,
        message: "Hola Mundo!".to_owned(),
        data: "".to_owned()
    };
    response.message = "Hola mundo!".to_owned();
    let json_response: String = serde_json::to_string(&response).unwrap();
    content::Json(json_response)

}

#[get("/hello/<name>")]
pub fn hello_name(name: String) -> String {
    format!("Hello, {}!", name.as_str())
}