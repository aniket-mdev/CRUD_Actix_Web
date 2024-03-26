use actix_web::web;
use api::book_api::*;

use crate::api;
pub fn book_router() -> actix_web::Scope{
    web::scope("book")
        .service(get_book)
        .service(create_book)            
        .service(get_books)
        .service(update_book)
        .service(delete_book)
        .service(get_author_books)
        .service(delete_books_by_author)
        
}