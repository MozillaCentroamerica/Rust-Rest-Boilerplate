#![feature(proc_macro_hygiene, decl_macro)]

extern crate frank_jwt;
#[macro_use] extern crate rocket;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;
//extern crate accord;
//#[macro_use] extern crate serde;
//#[macro_use] extern crate serde_json;

//use rocket::response::content;

//mod model { pub mod persona; }
//use model::persona::{Persona};

//mod model { pub mod response; }



mod models;
mod routes;

//use other::world;
#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": false,
        "reason": "Resource was not found."
    })
}#[catch(422)]
fn unprocessable_entity() -> JsonValue {
    json!({
        "status": false,
        "reason": "Datos ingresados invalidos."
    })
}

fn main() {
    rocket::ignite().mount("/", routes![
    routes::hello::hola,
    routes::hello::hello_name
    ]).mount("/user", routes![
    routes::user::auth
    ]).register(catchers![not_found,unprocessable_entity])
        .launch();
}