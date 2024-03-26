extern crate dotenv;
use actix_web:: Result;

use futures::stream::TryStreamExt; //add this

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
     Collection, Database
};
use crate::model::user_model::User;

pub struct MongoRepo {
    col: Collection<User>
}

impl MongoRepo {
    pub async fn init(db:Database) -> Self {

        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user:User) ->Result<InsertOneResult, Error> {
        let new_doc = User {
            id:None,
            name:new_user.name,
            location:new_user.location,
            title:new_user.title,
        };

        let user = self
            .col
            .insert_one(new_doc,None)
            .await
            .ok()
            .expect("Error creating user");

        Ok(user)
    } 

    pub async fn get_user(&self, id:&String) -> Result<User, Error> {
        let obj_id:ObjectId;
        match ObjectId::parse_str(id) {
            Ok(obj) => {
                obj_id = obj;
            },
            Err(_) => {
                return Err(Error::DeserializationError { message: format!("Invalid id has been passed") });
            }
        };
        let filter = doc! {"_id":obj_id};
        let user_details = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");


        if user_details.is_none() {
            return Err(Error::DeserializationError { message: format!("user not found") });
        }
        Ok(user_details.unwrap_or_default())    
    }

    pub async fn check_user_exists(&self, user_name:&String) -> Result<User, Error> {
        let filter =doc! {"name":user_name};

        let user = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user details");

        if user.is_none() {
            return Err(Error::DeserializationError { message: format!("user not found") });
        }

        Ok(user.unwrap_or_default())
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
        }
    
    pub async fn update_user(&self, id:&String, new_user:User) -> Result<UpdateResult, Error> {
        let obj_id:ObjectId;
        match ObjectId::parse_str(id) {
            Ok(id) => {
                obj_id = id
            },
            Err(_) => {
                return Err(Error::DeserializationError { message: format!("invalid id has been passed") });
            }
        }

        let filter = doc! {"_id":obj_id};
        let new_doc = doc! {
            "$set":{
                "name":new_user.name,
                "location":new_user.location,
                "title":new_user.title
            },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id:&ObjectId) -> Result<DeleteResult, Error> {
        let filter = doc! {"_id":id};
        let user_details = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");

        Ok(user_details)
    }

    pub async fn get_user_by_name(&self, name:&str) -> Result<User, Error> {
        let filter = doc! {"name": name};
        let result = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Failed to fetch user");

        match result {
            Some(user) => {Ok(user)},
            None => {
                return Err(Error::DeserializationError { message: format!("user not found") });
            },
        }
    }

}