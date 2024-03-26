use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug,Default, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub location: String,
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LoginUser {
    pub name:String
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UserData {
    pub id:String,
    pub name:String,
    pub location:String,
    pub title:String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token:Option<String>
}