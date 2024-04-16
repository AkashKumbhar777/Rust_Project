use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::model::product::Product;
use crate::controller::sql_helper::{create, get, update, delete}; // Import the CRUD functions from sql_helper.rs
use std::collections::HashMap;

// Create a product
pub async fn create_product(
    product_input: web::Json<Product>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let new_product_input = product_input.into_inner();
    println!("Inside create_product");

    // Assuming `product_name`, `product_description`, `image_url`, `specifications`, `created_at`, and `updated_at` are Strings or can be represented as &str
    // Convert Product struct to HashMap<&str, &str>
    let mut data = HashMap::new();
    data.insert("product_name", new_product_input.product_name.as_str());
    data.insert("product_description", new_product_input.product_description.as_deref().unwrap_or_default());
    let price_str = new_product_input.price.to_string(); // Temporary variable to hold the String
    data.insert("price", &price_str);
    data.insert("image_url", new_product_input.image_url.as_deref().unwrap_or_default());
    let specifications_str = new_product_input.specifications.as_ref().map(|s| s.to_string()).unwrap_or_default(); // Temporary variable to hold the String
    data.insert("specifications", &specifications_str);
    let created_at_str = new_product_input.created_at.to_string(); // Assuming conversion to String is necessary
    data.insert("created_at", &created_at_str);
    let updated_at_str = new_product_input.updated_at.to_string(); // Assuming conversion to String is necessary
    data.insert("updated_at", &updated_at_str);

    match create(pool.get_ref(), "product", &data).await { // Call create function from sql_helper
        Ok(_) => HttpResponse::Ok().json(new_product_input),
        Err(e) =>{ 
            print!("{}", e);
            HttpResponse::InternalServerError().finish()},
    }
}


// Get all products
pub async fn get_products(pool: web::Data<PgPool>) -> impl Responder {
    match get(pool.get_ref(), "product", "", &["product_id", "product_name", "product_description", "price", "image_url", "specifications", "created_at", "updated_at"]).await { // Call get function from sql_helper
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

// Get product by its id
pub async fn get_product_by_product_id(
    product_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let product_id = product_id.into_inner();
    match get(pool.get_ref(), "product", &product_id.to_string(), &["product_id", "product_name", "product_description", "price", "image_url", "specifications", "created_at", "updated_at"]).await { // Call get function from sql_helper
        Ok(products) => {
            if let Some(product) = products.get(0) {
                HttpResponse::Ok().json(product)
            } else {
                HttpResponse::NotFound().body("Product not found")
            }
        },
        Err(err) => {
            println!("Failed to retrieve product: {}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}

// Update product by using its id
pub async fn update_product(
    product_id: web::Path<i32>,
    product_input: web::Json<Product>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let updated_product_input = product_input.into_inner();
    let product_id = product_id.into_inner();

    // Convert Product struct to HashMap<&str, &str>
    let mut data = HashMap::new();
    data.insert("product_name", updated_product_input.product_name.as_str());
    data.insert("product_description", updated_product_input.product_description.as_deref().unwrap_or_default());
    let price_str = updated_product_input.price.to_string(); // Temporary variable to hold the String
    data.insert("price", &price_str);
    data.insert("image_url", updated_product_input.image_url.as_deref().unwrap_or_default());
    let specifications_str = updated_product_input.specifications.as_ref().map(|s| s.to_string()).unwrap_or_default(); // Temporary variable to hold the String
    data.insert("specifications", &specifications_str);
    let created_at_str = updated_product_input.created_at.to_string(); // Assuming conversion to String is necessary
    data.insert("created_at", &created_at_str);
    let updated_at_str = updated_product_input.updated_at.to_string(); // Assuming conversion to String is necessary
    data.insert("updated_at", &updated_at_str);

    match update(pool.get_ref(), "product", &product_id.to_string(), &data).await { // Call update function from sql_helper with corrected data type
        Ok(_) => HttpResponse::Ok().json(updated_product_input),
        Err(err) => {
            println!("Failed to update product: {}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}

// Delete product
pub async fn delete_product(
    product_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let product_id = product_id.into_inner();
    match delete(pool.get_ref(), "product", &product_id.to_string()).await { // Call delete function from sql_helper
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
