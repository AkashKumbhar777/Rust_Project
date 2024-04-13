use actix_web::{web, HttpResponse, Responder};
use sqlx:: PgPool;
use crate::model::product::Product;
// use crate::model::models::Product;

pub async fn create_product(product_input: web::Json<Product>, pool: web::Data<PgPool>) -> impl Responder {
    let new_product_input = product_input.into_inner();
println!("Inside create_product");
    let result = sqlx::query(
        "INSERT INTO product (product_name, product_description, price, image_url, specifications, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind(&new_product_input.product_name)
        .bind(&new_product_input.product_description)
        .bind(new_product_input.price)
        .bind(&new_product_input.image_url)
        .bind(&new_product_input.specifications)
        .bind(&new_product_input.created_at)
        .bind(&new_product_input.updated_at)
        .execute(pool.as_ref()) // Use `as_ref()` to get a reference to the connection pool
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(new_product_input), // Return the created product if successful
        Err(_) => HttpResponse::InternalServerError().finish(), // Return internal server error if failed
    }
}


pub async fn get_products(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Product>("SELECT product_id, product_name, product_description, price::FLOAT8 as price, image_url, specifications, created_at, updated_at FROM product")
        .fetch_all(pool.as_ref())
        .await
    {
        Ok(products) => {
            println!("Retrieved {} products", products.len());
            HttpResponse::Ok().json(products)
        },
        Err(err) => {
            println!("Failed to retrieve products: {}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}



