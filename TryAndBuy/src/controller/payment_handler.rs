use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::model::payment::Payment; // Assuming the Payments struct is defined in the same module as the handler

// Create Payment
pub async fn create_payment(
    payment_input: web::Json<Payments>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let new_payment_input = payment_input.into_inner();
    let result = sqlx::query(
        "INSERT INTO payments (user_id, order_id, payment_id, currency, total_amount, payment_status)
         VALUES ($1, $2, $3, $4, $5, $6)")
         .bind(&new_payment_input.user_id)
         .bind(&new_payment_input.order_id)
         .bind(&new_payment_input.payment_id)
         .bind(&new_payment_input.currency)
         .bind(&new_payment_input.total_amount)
         .bind(&new_payment_input.payment_status)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(new_payment_input),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Get all Payments
pub async fn get_all_payments(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Payments>(
        "SELECT reciept_no, user_id, order_id, payment_id, currency, total_amount, payment_status
         FROM payments")
        .fetch_all(pool.as_ref())
        .await
    {
        Ok(payments) => HttpResponse::Ok().json(payments),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Get Payment by receipt number
pub async fn get_payment_by_receipt_number(
    receipt_no: web::Path<i32>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let receipt_no = receipt_no.into_inner();

    match sqlx::query_as::<_, Payments>(
        "SELECT reciept_no, user_id, order_id, payment_id, currency, total_amount, payment_status
         FROM payments
         WHERE reciept_no = $1")
        .bind(&receipt_no)
        .fetch_optional(pool.as_ref())
        .await
    {
        Ok(Some(payment)) => HttpResponse::Ok().json(payment),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Update Payment
pub async fn update_payment(
    receipt_no: web::Path<i32>,
    payment_input: web::Json<Payments>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let receipt_no = receipt_no.into_inner();
    let updated_payment_input = payment_input.into_inner();

    let result = sqlx::query(
        "UPDATE payments
         SET user_id = $1, order_id = $2, payment_id = $3, currency = $4, total_amount = $5, payment_status = $6
         WHERE reciept_no = $7")
        .bind(&updated_payment_input.user_id)
        .bind(&updated_payment_input.order_id)
        .bind(&updated_payment_input.payment_id)
        .bind(&updated_payment_input.currency)
        .bind(&updated_payment_input.total_amount)
        .bind(&updated_payment_input.payment_status)
        .bind(&receipt_no)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(updated_payment_input),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Delete Payment
pub async fn delete_payment(
    receipt_no: web::Path<i32>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let receipt_no = receipt_no.into_inner();

    let result = sqlx::query(
        "DELETE FROM payments
         WHERE reciept_no = $1")
         .bind(&receipt_no)
         .execute(pool.as_ref())
         .await;
 
     match result {
         Ok(_) => HttpResponse::Ok().finish(),
         Err(_) => HttpResponse::InternalServerError().finish(),
     }
 }
