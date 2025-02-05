use std::{future::{ready, Future, Ready}, pin::Pin};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, error::ErrorUnauthorized, Error
};
use crate::middleware::jwt_config;

pub struct Authentication;

impl<S,B> Transform<S, ServiceRequest> for Authentication 
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S>{
    service:S
}

type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

impl<S,B> Service<ServiceRequest> for AuthenticationMiddleware<S> 
where
    S:Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    forward_ready!(service);


    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("req path : {}", req.path());
        if req.path() != "/user/login"  {
            let verify = req.headers().get("Authorization");

            if verify.is_none() {
                return Box::pin(async move{
                    Err(ErrorUnauthorized("Token not found"))
                });
            }
    
            if let Some(auth_data) = verify {
                let data = auth_data.to_str();
                match data {
                    Ok(auth_val) => {
                        let header_val = format!("{}", auth_val);
                        let token_author = jwt_config::TokenAuthentication::init();
    
                        match token_author.validate_token(&header_val) {
                            Ok(_) => {},
                            Err(e) => {
                                return Box::pin(async move{
                                    Err(ErrorUnauthorized(e.to_string()))
                                });
                            },
                        }
                    },
                    Err(_) => {
                        return Box::pin(async move {
                            Err(ErrorUnauthorized("UnAuthorised Request"))
                        });
                    }
                }
            }
    
        }


        let fut = self.service.call(req);

        Box::pin(async move{
            let res = fut.await?;
            Ok(res)
        })

    }
}