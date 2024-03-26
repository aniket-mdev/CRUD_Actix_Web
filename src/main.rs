
use actix_web::{HttpResponse, Responder};
use actix_web::{middleware::Logger, web::Data, App, HttpServer, web,web::Path
};
use std::io::Write;
use actix_multipart::Multipart;
use futures::StreamExt;
mod api;
mod model;
mod repository;
mod middleware;
pub mod utils;
pub mod routers;
pub mod config;
use repository::mongodb_repo::MongoRepo;
use repository::book_repo::BookRepo;
use  repository::*;
use routers::{user_router::user_router, book_router::book_router, author_router::author_router};
use config::db_config::DBConfig;
use std::fs::File;
use std::io::Read;

async fn handle_multipart(mut payload: Multipart) -> impl Responder {
    // Iterate over multipart fields
    println!("handle multipart called..");
    if let Some(item) = payload.next().await {
        let mut field = item.expect("Failed to get next field from multipart request");
        
        let content_disposition = field.content_disposition();
        
        let file_name = content_disposition.get_filename();
        
        if let Some(file_val) = file_name {
            let filepath = format!("./tmp/{}", file_val);

            match  std::fs::File::create(filepath) {
                Ok(mut file) => {
                    while let Some(chunk) = field.next().await {
                        let data = chunk.expect("failed to read filed data chunk");
                        let err = file.write_all(&data);
                        println!("File Write Error : {:?}", err);
                        println!("File Name : {:#?}", file)
                    }
                },
                Err(e) => {
                    println!("{:?}",e.to_string())
                },
            }
        }
    }
    HttpResponse::Ok().finish()
}


async fn index(path:Path<(String,String)>) -> HttpResponse {

    let (dirctory,file_name) = path.into_inner();
    let file_path = format!("./{}/{}",dirctory, file_name);
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return HttpResponse::NotFound().body(e.to_string()),
    };
    
    let mut buffer = Vec::new();
    if let Err(_) = file.read_to_end(&mut buffer) {
        return HttpResponse::InternalServerError().finish();
    }

    // Return the image in the response body
    HttpResponse::Ok()
        .content_type("image/jpeg")
        .body(buffer)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }   
    env_logger::init();


    let db = DBConfig::init().await;
    let book_db = BookRepo::init(db.clone()).await;
    let user_db = MongoRepo::init(db.clone()).await;
    let author_db = author_repo::AuthorRepo::init(db).await;
    let db_book_db = Data::new(book_db);
    let db_user_db = Data::new(user_db);
    let db_author = Data::new(author_db);
    
    
    println!("🚀 Server started successfully");
    // let files: HashMap<&str, static_files::Resource> = HashMap::new();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::auth_middleware::Authentication)
            .app_data(db_book_db.clone())
            .app_data(db_user_db.clone())
            .app_data(db_author.clone())
            .service(user_router())
            .service(book_router())
            .service(author_router())
            .route("/upload", web::post().to(handle_multipart))
            .route("/get_file/{dirc}/{file_name}", web::get().to(index))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
