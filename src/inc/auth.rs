
use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

pub extern crate crypto;
pub extern crate jwt;
pub extern crate rustc_serialize;
use serde_json::{json as sjson, Value};


use rocket_contrib::json::JsonValue;

use crate::models::structs::User;
