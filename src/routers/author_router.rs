use actix_web::web;
use crate::api::author_api::*;

pub fn author_router() -> actix_web::Scope {
    web::scope("author")
        .route("/author", web::post().to(create_author))
        .route("/author/{id}", web::get().to(get_author))
        .route("/authors", web::get().to(get_authors))
        .route("/author/{id}", web::put().to(update_author))
        .route("/author/{id}", web::put().to(delete_author))
        .route("/author/profile", web::post().to(upload_author_profile))
        .route("/auther_profile/{id}", web::get().to(get_author_profile))
        .route("/get_file/{directory}/{file_name}", web::get().to(get_files))

}
