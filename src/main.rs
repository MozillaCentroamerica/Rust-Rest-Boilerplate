#![feature(proc_macro_hygiene, decl_macro)]

extern crate frank_jwt;
#[macro_use] extern crate rocket;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;



mod models;
mod routes;
mod inc;


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
    rocket::ignite().
        mount("/", routes![
    routes::hello::hola,
    routes::hello::hello_name
    ]).mount("/user", routes![
    routes::user::auth
    routes::user::me
    ]).
        register(catchers![not_found,unprocessable_entity])
        .launch();
}