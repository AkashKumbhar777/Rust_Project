use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::model::refund_return::RefundReturn;

// Create RefundReturn
pub async fn create_refund_return(
    refund_return_input: web::Json<RefundReturn>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let new_refund_return_input = refund_return_input.into_inner();
    let result = sqlx::query(
        "INSERT INTO refund_return (order_id, reason, refunded_amount, refund_date, payment_id)
         VALUES ($1, $2, $3, $4, $5)")
         .bind(&new_refund_return_input.order_id)
         .bind(&new_refund_return_input.reason)
         .bind(&new_refund_return_input.refunded_amount)
         .bind(&new_refund_return_input.refund_date)
         .bind(&new_refund_return_input.payment_id)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(new_refund_return_input),
        Err(e) => {println!("\n\nError  : {:?} \n\n",e);
        HttpResponse::InternalServerError().finish()},
    }
}

// Get all RefundReturns
pub async fn get_all_refund_returns(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, RefundReturn>(
        "SELECT refund_id, order_id, reason, refunded_amount, refund_date, payment_id
         FROM refund_return")
        .fetch_all(pool.as_ref())
        .await
    {
        Ok(refund_returns) => HttpResponse::Ok().json(refund_returns),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Get RefundReturn by refund_id
pub async fn get_refund_return_by_id(
    refund_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let refund_id = refund_id.into_inner();

    match sqlx::query_as::<_, RefundReturn>(
        "SELECT refund_id, order_id, reason, refunded_amount, refund_date, payment_id
         FROM refund_return
         WHERE refund_id = $1")
        .bind(&refund_id)
        .fetch_optional(pool.as_ref())
        .await
    {
        Ok(Some(refund_return)) => HttpResponse::Ok().json(refund_return),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Update RefundReturn
pub async fn update_refund_return(
    refund_id: web::Path<i32>,
    refund_return_input: web::Json<RefundReturn>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let refund_id = refund_id.into_inner();
    let updated_refund_return_input = refund_return_input.into_inner();

    let result = sqlx::query(
        "UPDATE refund_return
         SET order_id = $1, reason = $2, refunded_amount = $3, refund_date = $4, payment_id = $5
         WHERE refund_id = $6")
        .bind(&updated_refund_return_input.order_id)
        .bind(&updated_refund_return_input.reason)
        .bind(&updated_refund_return_input.refunded_amount)
        .bind(&updated_refund_return_input.refund_date)
        .bind(&updated_refund_return_input.payment_id)
        .bind(&refund_id)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(updated_refund_return_input),
        Err(e) => {println!("Error  : {:?}",e);
            HttpResponse::InternalServerError().finish()},
    }
}

// Delete RefundReturn
pub async fn delete_refund_return(
    refund_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let refund_id = refund_id.into_inner();

    let result = sqlx::query(
        "DELETE FROM refund_return
         WHERE refund_id = $1")
        .bind(&refund_id)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
