use actix_web::{ dev::{ Service, ServiceRequest, ServiceResponse, Transform }, Error, HttpMessage };
use futures_util::future::{ ok, Ready, LocalBoxFuture };
use std::rc::Rc;

use crate::errors::AppError;
use crate::models::user::User; // adjust based on your user model path

pub struct AdminMiddleware;

impl<S, B> Transform<S, ServiceRequest>
    for AdminMiddleware
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
        B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AdminMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AdminMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct AdminMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest>
    for AdminMiddlewareService<S>
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
        let srv = self.service.clone();

        Box::pin(async move {
            let is_admin = {
                if let Some(user) = req.extensions().get::<User>() {
                    user.role == "admin" // or `user.is_admin == true`
                } else {
                    return Err(AppError::Unauthorized("User not authenticated".into()).into());
                }
            };

            if is_admin {
                srv.call(req).await
            } else {
                Err(AppError::Forbidden("Admin access required".into()).into())
            }
        })
    }
}
