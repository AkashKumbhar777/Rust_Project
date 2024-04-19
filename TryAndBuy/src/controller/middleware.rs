// use actix_service::{Service, Transform};
// use actix_web::{
//     dev::{ServiceRequest, ServiceResponse},
//     Error, HttpRequest,
// };
// use futures::future::{ok, Ready};
// use crate::controller::auth::fetch_user_info;
// pub struct AuthenticationMiddleware;

// impl<S: 'static> Transform<S, ServiceRequest> for AuthenticationMiddleware
// where
//     S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
// {
//     type Request = ServiceRequest;
//     type Response = ServiceResponse;
//     type Error = Error;
//     type Transform = AuthenticationMiddlewareService<S>;
//     type InitError = ();
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;

//     fn new_transform(&self, service: S) -> Self::Future {
//         ok(AuthenticationMiddlewareService { service })
//     }
// }

// pub struct AuthenticationMiddlewareService<S> {
//     service: S,
// }

// impl<S, B> Service<ServiceRequest> for AuthenticationMiddlewareService<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: actix_web::body::MessageBody,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = std::pin::Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>>>>;

//     fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(cx)
//     }

//     fn call(&mut self, req: ServiceRequest) -> Self::Future {
//         let fut = self.service.call(req);

//         Box::pin(async move {
//             // Extract the JWT token from the request headers
//             let authorization_header = req.headers().get("Authorization");
//             let jwt_token = match authorization_header {
//                 Some(header_value) => {
//                     match header_value.to_str() {
//                         Ok(token_str) => {
//                             if token_str.starts_with("Bearer ") {
//                                 Some(token_str.trim_start_matches("Bearer ").to_owned())
//                             } else {
//                                 None
//                             }
//                         },
//                         Err(_) => None,
//                     }
//                 },
//                 None => None,
//             };

//             // Validate the JWT token and fetch user information
//             let user_info = match jwt_token {
//                 Some(token) => fetch_user_info(&token).await,
//                 None => return Err(actix_web::error::ErrorUnauthorized("No JWT token provided").into()),
//             };

//             match user_info {
//                 Ok(user_info) => {
//                     // Attach user information to the request for use in route handlers
//                     let mut req = req;
//                     req.extensions_mut().insert(user_info);
//                     // Proceed with the request
//                     fut.await
//                 },
//                 Err(_) => {
//                     // If user info is not successfully obtained, return an Unauthorized response
//                     Err(actix_web::error::ErrorUnauthorized("Unauthorized").into())
//                 }
//             }
//         })
//     }
// }