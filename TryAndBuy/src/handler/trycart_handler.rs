use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::model::TryCart; // Assuming the TryCart struct is defined in the same module as Product

// Create TryCart
pub async fn create_try_cart(
    try_cart_input: web::Json<TryCart>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let new_try_cart_input = try_cart_input.into_inner();

    let result = sqlx::query(
        "INSERT INTO try_cart (user_id, product_id, added_at)
         VALUES ($1, $2, $3)")
        .bind(&new_try_cart_input.user_id)
        .bind(&new_try_cart_input.product_id)
        .bind(&new_try_cart_input.added_at)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(new_try_cart_input),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_all_try_carts(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, TryCart>(
        "SELECT try_cart_id, user_id, product_id, added_at
         FROM try_cart")
        .fetch_all(pool.as_ref())
        .await
    {
        Ok(try_carts) => HttpResponse::Ok().json(try_carts),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


// Get TryCarts by User ID
pub async fn get_try_cart_by_user_id(
    user_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let user_id = user_id.into_inner();

    match sqlx::query_as::<_, TryCart>(
        "SELECT try_cart_id, user_id, product_id, added_at
         FROM try_cart
         WHERE user_id = $1")
        .bind(&user_id)
        .fetch_all(pool.as_ref())
        .await
    {
        Ok(try_carts) => HttpResponse::Ok().json(try_carts),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Update TryCart
pub async fn update_try_cart(
    try_cart_id: web::Path<i32>,
    try_cart_input: web::Json<TryCart>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let try_cart_id = try_cart_id.into_inner();
    let updated_try_cart_input = try_cart_input.into_inner();

    let result = sqlx::query(
        "UPDATE try_cart
         SET product_id = $1, added_at = $2
         WHERE try_cart_id = $3")
        .bind(&updated_try_cart_input.product_id)
        .bind(&updated_try_cart_input.added_at)
        .bind(&try_cart_id)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(updated_try_cart_input),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Delete TryCart
pub async fn delete_try_cart(
    try_cart_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let try_cart_id = try_cart_id.into_inner();

    let result = sqlx::query(
        "DELETE FROM try_cart
         WHERE try_cart_id = $1")
        .bind(&try_cart_id)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
