use mongodb::{Client, Database};
use std::env;
extern crate dotenv;
use dotenv::dotenv;



pub struct DBConfig {
    db:Database
}

impl DBConfig {
    pub async fn init() -> Database {
        dotenv().ok();

        let uri = match env::var("MONGOURI") {
          Ok(v) => v.to_string(),
          Err(_) => format!("Error loading env variable"),  
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        db
        // let col:Collection<User> = db.collection("User");
        // DBConfig {col}
    }

    pub async fn connection(&mut self) {
        dotenv().ok();

        let uri = match env::var("MONGOURI") {
          Ok(v) => v.to_string(),
          Err(_) => format!("Error loading env variable"),  
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        self.db = db
    }

    // pub async fn get_collection(collection_name:String) -> Collation<T> {
    //     let col = Self::db.collection(collection_name);
    //     col
    // }
}