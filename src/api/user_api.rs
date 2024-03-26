use crate::{middleware, model::{self, user_model::{LoginUser, User, UserData}}, repository::mongodb_repo::MongoRepo};
use actix_web::{ delete, get, post, put, web::{Data, Json, Path}, HttpResponse};
use crate::utils::response::ResponseBuilder;
use mongodb::bson::oid::ObjectId;

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) ->HttpResponse {
    // check the user is exists or not
    let req_name  = &new_user.name;
    let old_user = db.check_user_exists(req_name).await;
    match old_user {
        // if ok means user already exists
        Ok(_) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(String::from("user already exists"));
            HttpResponse::InternalServerError().body(serde_json::to_string(&response).unwrap())
        },
        Err(_) => {
            
            let data = User {
                id:None,
                name:new_user.name.to_owned(),
                location: new_user.location.to_owned(),
                title: new_user.title.to_owned(),
            };
        
            let user_detail = db.create_user(data).await;
            match user_detail {
                Ok(user) => {
                    let response = ResponseBuilder::BuildSuccessResponse(String::from("User account has been created"), Some(user));
                    HttpResponse::Ok().json(response)
                },
                Err(err) => {
                    let response = ResponseBuilder::<()>::BuildFailedResponse(err.to_string());
                    HttpResponse::InternalServerError().json(response)
                },
            }
        }
    }
    

}

#[get("/user/{id}")]
pub async fn get_user(db:Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid id");
    }
    let user_details = db.get_user(&id).await;
    match user_details  {
        Ok(user) => {
            let  user_data = model::user_model::UserData{
                id:user.id.unwrap().to_hex(),
                name:user.name,
                location:user.location,
                title:user.title,
                access_token:None,
            };

            let response = ResponseBuilder::<model::user_model::UserData>::BuildSuccessResponse(String::from("user fetch successfully"), Some(user_data));
           return  HttpResponse::Ok().body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(err.to_string());

            if  err.to_string() == "Invalid id has been passed" {
               return  HttpResponse::BadRequest().body(serde_json::to_string(&response).unwrap());
            }
            if err.to_string() == "user not found" {
                return HttpResponse::NotFound().body(serde_json::to_string(&response).unwrap());
            }
            HttpResponse::InternalServerError().body(serde_json::to_string(&response).unwrap())
        }
    }   
    
}

#[get("/users")]
pub async fn get_users(db:Data<MongoRepo>) -> HttpResponse {
    let users = db.get_all_users().await;
    // println!("Users : {:?}", users);
    let n = users.as_ref().unwrap().len() ;
    if n as i32 == 0 {
        let response = ResponseBuilder::<()>::BuildFailedResponse(String::from("Users not found"));
        return HttpResponse::NotFound().body(serde_json::to_string(&response).unwrap());
    }
    let result = users.unwrap();
    let mut response_users:Vec<UserData> = Vec::new();
    for user in result.iter() {
        // println!("user name : {:?}", user.name);
       let user_data = UserData {
            id:user.id.unwrap().to_hex(),
            name:user.name.to_string(),
            location:user.location.to_string(),
            title:user.title.to_string(),
            access_token:None,
       };
       response_users.push(user_data)
    }
    let response = ResponseBuilder::<Vec<model::user_model::UserData>>::BuildSuccessResponse(String::from("users fetched!"), Some(response_users));
    HttpResponse::Ok().json(response)
}

#[put("/user/{id}")]
pub async fn update_user(db:Data<MongoRepo>, path: Path<String>,new_user:Json<User>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };

    let obj_id:ObjectId;

    match ObjectId::parse_str(&id) {
        Ok(obj) => {
            obj_id = obj
        },
        Err(_) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(String::from("invalid id"));
            return HttpResponse::BadRequest().json(response);
        }
    }

    let data = User {
        id:Some(obj_id),
        name: new_user.name.to_owned(),
        location:new_user.location.to_owned(),
        title:new_user.title.to_owned(),
    };

    let update_result = db.update_user(&id, data).await;

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;
                return match updated_user_info {
                    Ok(user) => {
                        let user_data = UserData {
                            id:user.id.unwrap().to_hex(),
                            name:user.name,
                            location:user.location,
                            title:user.title,
                            access_token:None,
                        };
                        let response = ResponseBuilder::BuildSuccessResponse(String::from("user update successfully!"), Some(user_data));
                        HttpResponse::Ok().json(response)
                    },
                    Err(err) => {
                        let response = ResponseBuilder::<()>::BuildFailedResponse(err.to_string());
                        HttpResponse::InternalServerError().json(response)
                    },
                };
            }
            let response = ResponseBuilder::<()>::BuildFailedResponse(String::from("No user found with specific id"));
            return HttpResponse::NotFound().json(response);
        }
        Err(err) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(String::from(err.to_string()));
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[delete("/user/{id}")]
pub async fn delete_user(db:Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let obj_id:ObjectId;
    let id = path.into_inner();

    if id.is_empty() {
        let response = ResponseBuilder::<()>::BuildFailedResponse("invalid ID".to_string());
        HttpResponse::BadRequest().json(response);
    }

    match ObjectId::parse_str(&id) {
        Ok(obj) => {
            obj_id = obj
        },
        Err(_)=> {
            let response = ResponseBuilder::<()>::BuildFailedResponse(String::from("invalid id"));
            return HttpResponse::BadRequest().json(response);
        }
    }

    let user_details = db.delete_user(&obj_id).await;
    return match user_details {
        Ok(result) => {
            if result.deleted_count == 1 {
                let response = ResponseBuilder::<()>::BuildSuccessResponse(String::from("User successfully deleted"), None);
                return HttpResponse::Ok().json(response)
            }
            let response = ResponseBuilder::<()>::BuildFailedResponse(String::from("failed to delete"));
            HttpResponse::BadRequest().json(response)
        },
        Err(err) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(String::from(err.to_string()));
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[post("/login")]
pub async fn login_user(db:Data<MongoRepo>, user_data:Json<LoginUser>) -> HttpResponse {
    let name = user_data.name.to_owned();

    let result = db.get_user_by_name(&name).await;
    
    match result {
        Ok(user) => {
            // if user found then generate a token
            let mut user_data = UserData { id: user.id.unwrap().to_string(), name, location: user.location, title: user.title, access_token:None };
            let token_authentication = middleware::jwt_config::TokenAuthentication::init();
            let token = token_authentication.generate_token(&user_data);
            user_data.access_token = Some(token);
            HttpResponse::Ok().json(user_data)
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(e.to_string())
        },
    }

}