use actix_web::{ dev::{ Service, ServiceRequest, ServiceResponse, Transform }, Error, HttpMessage };
use futures_util::future::{ ok, Ready, LocalBoxFuture };
use std::rc::Rc;
use crate::{ config::Config, errors::AppError };
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
        let auth_header = req.headers().get(AUTHORIZATION).cloned();

        let srv = self.service.clone();
        Box::pin(async move {
            if let Some(header_value) = auth_header {
                let token_str = header_value.to_str().unwrap_or("");
                if token_str.starts_with("Bearer ") {
                    let token = &token_str[7..];

                    let config = Config::from_env();

                    if let Ok(user) = crate::auth::jwt::decode_jwt(token, &config.jwt_secret) {
                        req.extensions_mut().insert(user);
                    }

                    return srv.call(req).await;
                }
            }
            Err(AppError::Unauthorized("Unauthorized".into()).into())
        })
    }
}
