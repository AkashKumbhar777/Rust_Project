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
    match sqlx::query_as::<_, Product>("SELECT product_id, product_name, product_description, price::FLOAT8 as price, image_url, specifications, created_at, updated_at, categary FROM product")
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

pub async fn get_product_by_product_id(
    product_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let product_id = product_id.into_inner();
    match sqlx::query_as::<_, Product>("SELECT product_id, product_name, product_description, price::FLOAT8 as price, image_url, specifications, created_at, updated_at,categary FROM product WHERE product_id = $1")
        .bind(product_id)
        .fetch_optional(pool.as_ref())
        .await
    {
        Ok(product) => {
            if let Some(product) = product {
                HttpResponse::Ok().json(product)
            } else {
                HttpResponse::NotFound().body("Product not found")
            }
        }
        Err(err) => {
            println!("Failed to retrieve product: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_product(
    product_id: web::Path<i32>,
    product_input: web::Json<Product>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let product_id = product_id.into_inner();
    let updated_product_input = product_input.into_inner();
    let result = sqlx::query(
        "UPDATE product 
         SET product_name = $1, product_description = $2, price = $3, image_url = $4, specifications = $5, updated_at = $6
         WHERE product_id = $7",
    )
    .bind(&updated_product_input.product_name)
    .bind(&updated_product_input.product_description)
    .bind(updated_product_input.price)
    .bind(&updated_product_input.image_url)
    .bind(&updated_product_input.specifications)
    .bind(&updated_product_input.updated_at)
    .bind(product_id)
    .execute(pool.as_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(updated_product_input),
        Err(err) => {
            println!("Failed to retrieve product: {}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}


pub async fn delete_product(
    product_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let product_id = product_id.into_inner();
    let result = sqlx::query("DELETE FROM product WHERE product_id = $1")
        .bind(product_id)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


