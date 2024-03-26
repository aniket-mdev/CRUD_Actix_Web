use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id:Option<ObjectId>,
    pub book_name:String,
    pub book_author:String,
    pub total_page:i32
}