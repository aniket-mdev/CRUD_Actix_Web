use std::{fs::File, io::{Read, Write}};

use actix_multipart::Multipart;
use actix_web::{web::{Data, Json,Path}, HttpResponse, Responder};
use futures::StreamExt;
use mongodb::bson::oid::ObjectId;

use crate::model::{self, author_model::*};
use crate::utils::response::ResponseBuilder;
use crate::repository::*;

use self::author_repo::AuthorRepo;

use super::app_errors::{AppError, AppMessage};


pub async fn create_author(db:Data<AuthorRepo>, author:Json<Author>) -> impl Responder {
    let new_author = Author {
        id:None,
        author_name:author.author_name.to_owned(),
        author_email:author.author_email.to_owned(),
        contact:author.contact.to_owned(),
        created_at:None,
    };

    let author_result = db.create_author(new_author).await;
    match author_result {
        Ok(result) => {
            let response = ResponseBuilder::BuildSuccessResponse(AppMessage::InsertSuccessMsg.to_string(), Some(result));
            HttpResponse::Ok().json(response)
        },
        Err(err) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(err.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

pub async fn get_authors(db:Data<AuthorRepo>) -> impl Responder {
    let authors = db.get_authors().await;

    if authors.as_ref().unwrap().len() as i32 == 0 {
        return HttpResponse::NotFound().body(AppError::DataNotFoundError.to_string());
    }

    let mut result:Vec<AuthorData> = Vec::new();
    let data = authors.unwrap();
    for i in data {
        if !i.created_at.is_none() {
            let temp = AuthorData::set_data(i);
            result.push(temp);
        }
    }

    let response = ResponseBuilder::<Vec<model::author_model::AuthorData>>::BuildSuccessResponse(AppMessage::FetchSuccessMsg.to_string(), Some(result));
    HttpResponse::Ok().json(response)
}

pub async fn get_author(db:Data<AuthorRepo>, path: Path<String>) -> impl Responder {
    let id = path.into_inner();

    return match ObjectId::parse_str(id) {
        Ok(obj) => {
            let author_details = db.get_author(obj).await;
            match author_details {
                Ok(author) => {
                    let author_data = AuthorData::set_data(author);
                    let response = ResponseBuilder::BuildSuccessResponse("author found".to_string(), Some(author_data));
                    HttpResponse::Ok().json(response)
                },
                Err(err) => {
                    let response = ResponseBuilder::<()>::BuildFailedResponse(err.to_string());
                    if err.to_string() == AppError::DataNotFoundError.to_string() {
                        return HttpResponse::NotFound().json(response)
                    }
                    HttpResponse::InternalServerError().json(response)
                }
            }
        },
        Err(_) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(AppError::InvalidIdError.to_string());
            HttpResponse::BadRequest().json(response)
        }
    };
}

pub async fn update_author(db:Data<AuthorRepo>, path: Path<String>, author:Json<Author>) -> impl Responder {
    let id = path.into_inner();

    return match ObjectId::parse_str(id) {
        Ok(obj_id) => {
            let new_author = Author {
                id:None,
                author_name:author.author_name.to_owned(),
                author_email:author.author_email.to_owned(),
                contact:author.contact.to_owned(),
                created_at:None
            };

            let result = db.update_author(obj_id, new_author).await;

            match result {
                Ok(update_result) => {
                    if update_result.matched_count == 1 {
                        let response = ResponseBuilder::<()>::BuildSuccessResponse(AppMessage::UpdateSuccessMsg.to_string(), None);
                        return HttpResponse::Ok().json(response)
                    }
                    let response = ResponseBuilder::<()>::BuildFailedResponse(AppMessage::UpdateFailedMsg.to_string());
                    HttpResponse::BadRequest().json(response)
                },
                Err(e) => {
                    let response = ResponseBuilder::<()>::BuildFailedResponse(e.to_string());
                    HttpResponse::InternalServerError().json(response)
                }
            }
        },
        Err(_) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(AppError::InvalidIdError.to_string());
            HttpResponse::BadRequest().json(response)
        }
    };
}

pub async fn delete_author(db:Data<AuthorRepo>, path: Path<String>) -> impl Responder {
    let id = path.into_inner();

    return match ObjectId::parse_str(id) {
        Ok(obj_id) => {
            let delete_result = db.delete_author(obj_id).await;

            match delete_result {
                Ok(count) => {
                    if count.deleted_count == 1 {
                        let response = ResponseBuilder::<()>::BuildSuccessResponse(AppMessage::DeleteSuccessMsg.to_string(), None);
                        HttpResponse::Ok().json(response);
                    }
                    let response = ResponseBuilder::<()>::BuildFailedResponse(AppMessage::DeleteFailedMsg.to_string());
                    HttpResponse::BadRequest().json(response)
                },
                Err(e) => {
                    let response = ResponseBuilder::<()>::BuildFailedResponse(e.to_string());
                    HttpResponse::InternalServerError().json(response)
                }
            }
        },
        Err(_) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(AppError::InvalidIdError.to_string());
            HttpResponse::BadRequest().json(response)
        }
    };
}

pub async fn upload_author_profile(db:Data<AuthorRepo>, mut payload:Multipart) -> impl Responder {
    let mut author_profile = AuthorProfile {
        id: None,
        author_id: ObjectId::new(),
        profile_imgae: "".to_string(),
        created_at: None,
    };

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();

        if field.content_disposition().get_name() == Some("author_id") {
            let mut text = String::new();
            if let Some(content_data) = field.next().await {
                let data = content_data.unwrap();
                text.push_str(&String::from_utf8_lossy(&data));
            }
            
            // check the Auther ID
            match ObjectId::parse_str(text) {
                Ok(obj_id) => author_profile.author_id = obj_id,
                Err(_) => {
                    let response = ResponseBuilder::<()>::BuildFailedResponse("Invalid id provided".to_string());
                    return HttpResponse::BadRequest().json(response);
                },
            }
            
        }

        // check for file
        if field.content_disposition().get_name() == Some("file") {
            let file_name = field.content_disposition().get_filename();

            if let Some(file_val) = file_name {
                let file_path = format!("./tmp/{}", file_val);
                author_profile.profile_imgae = file_path.clone().chars().skip(1).collect();

                match std::fs::File::create(file_path) {
                    Ok(mut file) => {
                        while let Some(chunk) = field.next().await {
                            let data = chunk.unwrap();
                            if let Some(err) = file.write_all(&data).err(){
                                let response = ResponseBuilder::<()>::BuildFailedResponse(err.to_string());
                                return HttpResponse::InternalServerError().json(response);
                            }
                            
                        }
                    },
                    Err(e) => {
                        let response = ResponseBuilder::<()>::BuildFailedResponse(e.to_string());
                        return HttpResponse::UnprocessableEntity().json(response);
                    },
                }
            }
        }
        
    }
    let res = db.upload_profile_pic(author_profile).await;
    match res {
        Ok(_) => {
            let response = ResponseBuilder::<()>::BuildSuccessResponse("Author Profile has been uploaded".to_string(), None);
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(e.to_string());
            HttpResponse::InternalServerError().json(response)
        },
    }
}

pub async fn get_author_profile(db:Data<AuthorRepo>, path:Path<String>) -> impl Responder {
    let id = path.into_inner();

    return match ObjectId::parse_str(id) {
        Ok(obj_id)=>{
            let result = db.get_profile_pic(obj_id).await;
            match result {
                Ok(profile_data) => {
                    let author_profile_data = AuthorProfileData::set_profile_data(profile_data);
                    let response = ResponseBuilder::BuildSuccessResponse("Author Profile not found".to_string(), Some(author_profile_data));
                    HttpResponse::Ok().json(response)
                },
                Err(e) => {
                    let response = ResponseBuilder::<()>::BuildFailedResponse(e.to_string());
                    HttpResponse::BadRequest().json(response)
                },
            }
        },
        Err(_) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse("Invalid Id passed".to_string());
            HttpResponse::BadRequest().json(response)
        }
    };
}

pub async fn get_files(path:Path<(String, String)>) -> impl Responder {
    let (directory, file_name) = path.into_inner();
    let file_path = format!("./{}/{}", directory, file_name);

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return HttpResponse::NotFound().body(e.to_string()),
    };

    let mut buffer = Vec::new();
    if let Err(_) = file.read_to_end(&mut buffer) {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok()
        .content_type("image/jpeg")
        .body(buffer)
}