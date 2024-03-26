use actix_web::web;

use crate::api::user_api::*;

pub  fn user_router() -> actix_web::Scope {

    web::scope("user")
        .service(create_user)
        .service(get_user)
        .service(get_users)
        .service(update_user)
        .service(delete_user)
        .service(login_user)
}