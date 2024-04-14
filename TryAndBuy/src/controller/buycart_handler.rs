use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::model::buy_cart::BuyCart; // Assuming the BuyCart struct is defined in the same module as Product

// Create BuyCart
pub async fn create_buycart(
    user_id:web::Path<i32>,
    buy_cart_input: web::Json<BuyCart>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let new_buy_cart_input = buy_cart_input.into_inner();
    let user_id = user_id.into_inner();

    let result = sqlx::query(
        "INSERT INTO buy_cart (user_id, product_id, quantity, total_amount)
         VALUES ($1, $2, $3, $4)")
        .bind(&user_id)
        .bind(&new_buy_cart_input.product_id)
        .bind(&new_buy_cart_input.quantity)
        .bind(&new_buy_cart_input.total_amount)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(new_buy_cart_input),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Get all BuyCarts
pub async fn get_all_buycarts(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, BuyCart>(
        "SELECT buy_cart_id, user_id, product_id, quantity, total_amount
         FROM buy_cart")
        .fetch_all(pool.as_ref())
        .await
    {
        Ok(buy_carts) => HttpResponse::Ok().json(buy_carts),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Get BuyCarts by User ID
pub async fn get_buycart_by_user_id(
    user_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let user_id = user_id.into_inner();

    match sqlx::query_as::<_, BuyCart>(
        "SELECT buy_cart_id, user_id, product_id, quantity, total_amount
         FROM buy_cart
         WHERE user_id = $1")
        .bind(&user_id)
        .fetch_all(pool.as_ref())
        .await
    {
        Ok(buy_carts) => HttpResponse::Ok().json(buy_carts),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Update BuyCart
pub async fn update_buycart(
    path_params: web::Path<(i32, i32)>,
    buy_cart_input: web::Json<BuyCart>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let (user_id, buy_cart_id) = path_params.into_inner();
    let updated_buy_cart_input = buy_cart_input.into_inner();

    let result = sqlx::query(
        "UPDATE buy_cart
         SET product_id = $1, quantity = $2, total_amount = $3
         WHERE buy_cart_id = $4 and user_id = $5")
        .bind(&updated_buy_cart_input.product_id)
        .bind(&updated_buy_cart_input.quantity)
        .bind(&updated_buy_cart_input.total_amount)
        .bind(&buy_cart_id)
        .bind(&user_id)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(updated_buy_cart_input),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Delete BuyCart
pub async fn delete_buycart(
    path_params: web::Path<(i32,i32)>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let (user_id, buy_cart_id) = path_params.into_inner();

    let result = sqlx::query(
        "DELETE FROM buy_cart
         WHERE buy_cart_id = $1 and user_id = $2")
        .bind(&buy_cart_id)
        .bind(&user_id)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
