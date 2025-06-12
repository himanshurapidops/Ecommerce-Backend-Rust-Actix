use actix_web::{
    dev::{ Service, ServiceRequest, ServiceResponse, Transform },
    web,
    Error,
    HttpMessage,
};
use futures_util::future::{ ok, Ready, LocalBoxFuture };
use sqlx::PgPool;
use std::rc::Rc;
use crate::{ config::Config, errors::AppError, models::user::User };
use actix_web::http::header::AUTHORIZATION;

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest>
    for JwtMiddleware
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
        B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest>
    for JwtMiddlewareService<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
        B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("JWT middleware called");
        let auth_header = req.headers().get(AUTHORIZATION).cloned();

        let srv = self.service.clone();
        Box::pin(async move {
            if let Some(header_value) = auth_header {
                let token_str = header_value.to_str().unwrap_or("");
                if token_str.starts_with("Bearer ") {
                    let token = &token_str[7..];

                    let db = req
                        .app_data::<web::Data<PgPool>>()
                        .ok_or_else(||
                            AppError::InternalServerError("Database pool not found".into())
                        )?;

                    let config = Config::from_env();

                    if let Ok(user) = crate::utils::jwt::decode_jwt(token, &config.jwt_secret) {
                    println!("JWT middleware called 2");

                        let user_info = sqlx
                            ::query_as::<_, User>(
                                r#"
                        SELECT id, email, full_name, mobile, password, status, role, created_at
                        FROM users
                        WHERE id = $1
                        "#
                            )
                            .bind(&user.sub)

                            .fetch_one(db.as_ref()).await
                            .map_err(|_| AppError::Unauthorized("User not found".into()))?;

                        let x = req.extensions_mut().insert(user_info);
                        println!("{:#?}", x);
                    }

                    return srv.call(req).await;
                }
            }
            Err(AppError::Unauthorized("Unauthorized".into()).into())
        })
    }
}
