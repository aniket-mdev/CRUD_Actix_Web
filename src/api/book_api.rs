use actix_web::{ delete, get, post, put, web::{Data, Json, Path}, HttpResponse};
use mongodb::bson::oid::ObjectId;
use crate::{model, repository::book_repo::BookRepo};
use crate::model::book_model::Book;
use crate::utils::response::ResponseBuilder;

#[post("/book")]
pub async fn create_book(db:Data<BookRepo>, book:Json<Book>) -> HttpResponse {
    let data = Book {
        id:None,
        book_name:book.book_name.to_owned(),
        book_author:book.book_author.to_owned(),
        total_page:book.total_page.to_owned(),
    };

    let book_detail = db.create_book(data).await;
    match book_detail {
        Ok(book) => {
            let response = ResponseBuilder::BuildSuccessResponse(String::from("Book has been added"), Some(book));
            HttpResponse::Ok().json(response)
        },
        Err(err) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(String::from(err.to_string()));
            HttpResponse::InternalServerError().json(response)
        }
    } 
}

#[get("/book/{id}")]
pub async fn get_book(db:Data<BookRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    return match ObjectId::parse_str(id) {
        Ok(obj) => {
            let book_details = db.get_book(obj).await;
            match book_details {
                Ok(book) => {
                    let response = ResponseBuilder::BuildSuccessResponse(String::from("Book fetched!"), Some(book));
                    HttpResponse::Ok().json(response)
                },
                Err(err) => {
                    let response = ResponseBuilder::<()>::BuildFailedResponse(err.to_string());
                    HttpResponse::InternalServerError().json(response)
                }
            }
        },
        Err(_) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(String::from("invalid id found"));
            HttpResponse::BadRequest().json(response)
        }
    };
}

#[get("/books")]
pub async fn get_books(db:Data<BookRepo>) -> HttpResponse {
    let books = db.get_books().await;

    if books.as_ref().unwrap().len() as i32 == 0 {
        return HttpResponse::NotFound().body("Books not available");
    }
    let result = books.unwrap();
    let response = ResponseBuilder::<Vec<model::book_model::Book>>::BuildSuccessResponse(String::from("Books fetched !"), Some(result));
    HttpResponse::Ok().json(response)
}

#[put("/book/{id}")]
pub async fn update_book(db:Data<BookRepo>, path: Path<String>, book:Json<Book>) -> HttpResponse {
    let id = path.into_inner();

    return match ObjectId::parse_str(id) {
        Ok(obj)=>{

            let new_book = Book {
                id:None,
                book_name:book.book_name.to_owned(),
                book_author:book.book_author.to_owned(),
                total_page:book.total_page.to_owned(),
            };
            let update_result = db.update_book(obj, new_book).await;

            match update_result {
                Ok(update) => {
                    if update.matched_count == 1 {
                        let updated_book_info = db.get_book(obj).await;
                        match updated_book_info {
                            Ok(book) => {
                                let response = ResponseBuilder::BuildSuccessResponse(String::from("Book has been updated !"), Some(book));
                                return HttpResponse::Ok().json(response)
                            },
                            Err(err) => {
                                let response = ResponseBuilder::<()>::BuildFailedResponse(err.to_string()); 
                                return HttpResponse::InternalServerError().json(response)
                            }
                        }
                    }
                    let response = ResponseBuilder::<()>::BuildFailedResponse(String::from("no book found with id"));
                    HttpResponse::NotFound().json(response)

                },
                Err(err) => {
                    let response = ResponseBuilder::<()>::BuildFailedResponse(err.to_string());
                    HttpResponse::InternalServerError().json(response)
                }
            }

        },
        Err(_) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(String::from("Invalid id!"));
            HttpResponse::BadRequest().json(response)
        }
        
    };    
}

#[delete("/book/{id}")]
pub async fn delete_book(db:Data<BookRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();

    return match ObjectId::parse_str(id) {
        Ok(obj) => {
            let delete_book = db.delete_book(obj).await;
            match delete_book {
                Ok(result) => {
                    if result.deleted_count == 1 {
                        let response = ResponseBuilder::<()>::BuildSuccessResponse(String::from("Book has been deleted"), None);
                        return HttpResponse::Ok().json(response)
                    }
                    let response = ResponseBuilder::<()>::BuildFailedResponse(String::from("failed to delete book"));
                    HttpResponse::BadRequest().json(response) 
                },
                Err(err) => {
                    let response = ResponseBuilder::<()>::BuildFailedResponse(err.to_string());
                    HttpResponse::InternalServerError().json(response)
                }
            }
        },
        Err(_) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(String::from("invalid id"));
            HttpResponse::BadRequest().json(response)
        }
    };
}

#[get("/books/author/{id}")]
pub async fn get_author_books(db:Data<BookRepo>, path: Path<String>) -> HttpResponse {
    let author = path.into_inner();

    if author.trim().is_empty() {
        let response = ResponseBuilder::<()>::BuildFailedResponse("author id required".to_string());
        return HttpResponse::BadRequest().json(response);
    }

    let result = db.get_book_by_author(author).await;

    match result {
        Ok(books) => {
            if books.len() as i32 == 0 {
                let response = ResponseBuilder::<()>::BuildFailedResponse("Books Not Found".to_string());
                return HttpResponse::NotFound().json(response);
            }
            let response = ResponseBuilder::BuildSuccessResponse("Book fetched!".to_string(), Some(books));
            HttpResponse::Ok().json(response)
        },
        Err(err) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(err.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[delete("books/author/{id}")]
pub async fn delete_books_by_author(db:Data<BookRepo>, path: Path<String>) -> HttpResponse {
    let author = path.into_inner();
    if author.trim().is_empty() {
        let response = ResponseBuilder::<()>::BuildFailedResponse("author id required".to_string());
        return HttpResponse::BadRequest().json(response);
    }

    let result = db.delete_books_by_author(author).await;

    match result {
        Ok(count) => {
            if count.deleted_count == 1 {
                let response = ResponseBuilder::<()>::BuildSuccessResponse("books has been deleted".to_string(), None);
                return HttpResponse::Ok().json(response)
            }

            let response = ResponseBuilder::<()>::BuildFailedResponse("failed to delete books".to_string());
            HttpResponse::InternalServerError().json(response)
        },
        Err(err) => {
            let response = ResponseBuilder::<()>::BuildFailedResponse(err.to_string());
            HttpResponse::BadRequest().json(response)
        }
    }
}