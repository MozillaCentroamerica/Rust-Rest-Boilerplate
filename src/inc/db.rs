//ebRlSKzUcnsFCILd
//user

use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use std::env;
use dotenv::dotenv;

pub fn mongo_client(collection: String)-> mongodb::coll::Collection{
    dotenv().ok();
    let mut uri: String ="".to_string();
    let mut mongo_db: String ="".to_string();
    let mut mongo_host: String ="".to_string();
    let mut mongo_port: u16 = 27017;
    let mut mongo_user: String ="".to_string();
    let mut mongo_password: String ="".to_string();

    let mut mongodb: String ="".to_string();
    match env::var("MONGO_HOST") {
        Ok(val) => mongo_host = val,
        Err(e) => mongo_host = "localhost".to_string(),
    }
    match env::var("MONGO_PORT") {
        Ok(val) => mongo_port = val.parse().unwrap(),
        Err(e) => mongo_port = 27017,
    }
    match env::var("MONGO_DB") {
        Ok(val) => mongo_db = val,
        Err(e) => mongo_db = "test".to_string(),
    }

    match env::var("MONGO_USER") {
        Ok(val) => mongo_user = val,
        Err(e) => mongo_user = "test".to_string(),
    }

    match env::var("MONGO_PASSWORD") {
        Ok(val) => mongo_password = val,
        Err(e) => mongo_password = "test".to_string(),
    }


    let client = Client::connect(mongo_host.as_str(), mongo_port)
        .expect("Failed to initialize standalone client.");
    println!("MongoDB HOST {}",mongo_host);
    println!("MongoDB PORT {}",mongo_port);
    // create mongodb client
//    let client = Client::with_uri(uri.as_str()).expect("Failed to initialize standalone client.");
//    match  client{
//        Ok(client)=>{
//            println!("Conectado correctamente a MongoDB");
//        }, Err(e)=> {
//            println!("Error conectando a mongo");
//        }
//    }

    let db =client.db(mongo_db.as_str());
    db.auth(mongo_user.as_str(), mongo_password.as_str()).unwrap();

    let col = db.collection(collection.as_str());
    // create a cursor
//    let cursor = client.db(mongo_db.as_str())
//        .collection(collection.as_str())
//        .find(None, None)
//        .unwrap();
    return col;
}