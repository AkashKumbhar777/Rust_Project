use actix_web::{web, App, HttpServer};
use actix_web::http::header;
use actix_cors::Cors;
use std::convert::TryInto;
use crate::db_connect::db::get_pool;
use crate::controller::auth::authenticate;
mod controller;
mod db_connect;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool= get_pool().await.expect("Failed to Create Pool");

    print!("Inside Main Function");

    let cors_headers: Vec<header::HeaderName> = vec![
        header::AUTHORIZATION.try_into().unwrap(),
        header::ACCEPT.try_into().unwrap(),
        header::CONTENT_TYPE.try_into().unwrap(),
    ];

    HttpServer::new(move || {
        let cors_headers_clone = cors_headers.clone();
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(cors_headers_clone)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/authenticate").route(web::post().to(authenticate)))
            .service(web::resource("/admin/products").route(web::get().to(controller::admin::get_products)))
            .service(web::resource("/admin/product").route(web::post().to(controller::admin::create_product)))
            .service(web::resource("/user").route(web::post().to(controller::user_handler::create_user))) 
            .service(web::resource("/users").route(web::get().to(controller::user_handler::get_users)))
            .service(web::resource("/user/{id}").route(web::get().to(controller::user_handler::get_user_by_id)))
            .service(web::resource("/user/update/{id}").route(web::put().to(controller::user_handler::update_user)))
            .service(web::resource("/user/delete/{id}").route(web::delete().to(controller::user_handler::delete_user)))

            .service(web::resource("/trycart/{uid}").route(web::post().to(controller::trycart_handler::create_trycart)))
            .service(web::resource("/trycarts").route(web::get().to(controller::trycart_handler::get_all_trycarts))) 
            .service(web::resource("/trycart/{uid}").route(web::get().to(controller::trycart_handler::get_trycart_by_user_id)))
            .service(web::resource("/trycart/update/{uid}/{id}").route(web::put().to(controller::trycart_handler::update_trycart)))
            .service(web::resource("/trycart/delete/{uid}/{id}").route(web::delete().to(controller::trycart_handler::delete_trycart)))
            
            .service(web::resource("/buycart/{uid}").route(web::post().to(controller::buycart_handler::create_buycart)))
            .service(web::resource("/buycarts").route(web::get().to(controller::buycart_handler::get_all_buycarts)))
            .service(web::resource("/buycart/{uid}").route(web::get().to(controller::buycart_handler::get_buycart_by_user_id))) 
            .service(web::resource("/buycart/update/{uid}/{id}").route(web::put().to(controller::buycart_handler::update_buycart)))
            .service(web::resource("/buycart/delete/{uid}/{id}").route(web::delete().to(controller::buycart_handler::delete_buycart)))
            


    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
