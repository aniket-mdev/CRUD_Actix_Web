use serde::{Serialize,Deserialize};
use mongodb::bson::oid::ObjectId;
use mongodb::{bson, bson::Document};
#[derive(Default,Debug, Serialize, Deserialize)]
pub struct Author {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id:Option<ObjectId>,
    pub author_name:String,
    pub contact:String,
    pub author_email:String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at:Option<bson::DateTime>
}
#[derive(Default,Serialize, Deserialize)]
pub struct AuthorData {
    pub id:String,
    pub author_name:String,
    pub contact:String,
    pub author_email:String,
    pub created_at:String
}

impl AuthorData {
    
    pub fn set_data(author:Author) -> Self {
        AuthorData {
            id:author.id.unwrap().to_string(),
            author_name:author.author_name,
            author_email:author.author_email,
            contact:author.contact,
            created_at:author.created_at.unwrap().to_string(),
        }
    }
}

impl Author {
    // Function to convert Author struct to BSON Document
    pub fn to_document(&self) -> Result<Document, mongodb::bson::ser::Error> {
        bson::to_document(self)
    }

    // #[warn(unused_features)]
    // pub fn from_document(doc: Document) -> Result<Author, mongodb::bson::de::Error> {
    //     bson::from_document(doc)
    // }
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AuthorProfile {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id:Option<ObjectId>,
    pub author_id:ObjectId,
    pub profile_imgae:String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at:Option<bson::DateTime>
}

impl AuthorProfile {
    
    pub fn to_document(&self) -> Result<Document, mongodb::bson::ser::Error> {
        bson::to_document(self)
    }
}
#[derive(Default, Serialize, Deserialize)]
pub struct AuthorProfileData {
    pub id:String,
    pub author_id:String,
    pub profile_image:String,
    pub created_at:String
}

impl AuthorProfileData {
    pub fn set_profile_data(author:AuthorProfile) -> Self {
        let profile_img = format!("http://locathost:8000{}", author.profile_imgae);
        AuthorProfileData { id: author.id.unwrap().to_string(), author_id: author.author_id.to_string(), profile_image: profile_img.to_string(), created_at: author.created_at.unwrap().to_string() }
    }
}