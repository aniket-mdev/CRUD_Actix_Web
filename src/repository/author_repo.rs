use actix_web::Result;
use mongodb:: {
    bson::{ self, doc, extjson::de::Error,oid::ObjectId, Document},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection, Database
};
// use mongodb::error::Error;
use crate::{api::app_errors::AppError, model::author_model::{Author, AuthorProfile}};
use futures::stream::TryStreamExt; //add this

pub struct AuthorRepo {
    col :Collection<Document>,
    profile_col:Collection<Document>
} 


impl AuthorRepo {
    
    pub async fn init(db:Database) -> Self {
        let col:Collection<Document> = db.collection("Author");
        let profile_col:Collection<Document> = db.collection("Author_Profile");
        AuthorRepo{ col, profile_col }
    }

    pub async fn create_author(&self, mut author:Author) -> Result<InsertOneResult, Error> {
        let created_at_bson = bson::DateTime::now();

        if author.created_at.is_none() {
            author.created_at = Some(created_at_bson);
        }
        
        let bson_author = match author.to_document() {
            Ok(bson_doc) => bson_doc,
            Err(e) => {
                return Err(Error::DeserializationError { message: e.to_string() });
            }
        };
        
        let result = self.col.insert_one(bson_author, None).await;
        Ok(result.unwrap())
    }

    pub async fn get_authors(&self) -> Result<Vec<Author>, Error> {
        let mut cursor = self.col.find(None, None).await.ok().expect("Error fetch authors");

        let mut authors:Vec<Author> = Vec::new();
        while let Some(author) = cursor 
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")

        {
            authors.push(bson::from_document(author).unwrap());    
        }
        Ok(authors)
    }

    pub async fn get_author(&self, id:ObjectId) -> Result<Author, Error> {
        let filter = doc! {"_id": id};

        let author = match self.col.find_one(filter, None).await {
            Ok(author) => author,
            Err(e) => return Err(Error::DeserializationError{message:format!("Error fetching author by ID: {}", e)}),
        };
        if author.is_none() {
            return Err(Error::DeserializationError { message: AppError::DataNotFoundError.to_string() });
        }
        Ok(bson::from_document(author.unwrap_or_default()).unwrap())
    }

    pub async fn update_author(&self, id:ObjectId, author:Author) -> Result<UpdateResult, Error> {
        let filter = doc! {"_id":id};
        let created_at_bson = bson::DateTime::now();
       
        let update = doc! {
            "$set": {
                "author_name":author.author_name,  
                "author_email":author.author_email,
                "contact":author.contact,
                "updated_at":created_at_bson
            }
        };

        let result = self
            .col
            .update_one(filter, update, None)
            .await
            .ok()
            .expect("Error updating Author");

        Ok(result)

    }

    pub async fn delete_author(&self, id:ObjectId) -> Result<DeleteResult,Error> {
        let filter = doc! {"_id":id};

        let delete = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error delete author");

        Ok(delete)
    }

    pub async fn upload_profile_pic(&self, mut author:AuthorProfile) -> Result<InsertOneResult, Error> {
        let created_at_bson = bson::DateTime::now();

        if author.created_at.is_none() {
            author.created_at = Some(created_at_bson);
        }

        let bson_author = match author.to_document() {
            Ok(bson_doc) => bson_doc,
            Err(e) => {
                return Err(Error::DeserializationError { message: e.to_string() });
            }
        };

        let result = self.profile_col.insert_one(bson_author, None).await;
        Ok(result.unwrap())
    }

    pub async fn get_profile_pic(&self, id:ObjectId) ->Result<AuthorProfile, Error> {
        let filter = doc! {"author_id":id};

        let author_profile = match self.profile_col.find_one(filter, None).await {
            Ok(author_p)=> author_p,
            Err(e) => {return Err(Error::DeserializationError { message: e.to_string() });}
            
        };

        if author_profile.is_none() {
            return Err(Error::DeserializationError { message: AppError::UserNotFoundError.to_string() });
        }
        Ok(bson::from_document(author_profile.unwrap_or_default()).unwrap())
    }

}