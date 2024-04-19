use sqlx:: Error as SqlxError;
use sqlx::PgPool;
use actix_web::{web,web::Data, Responder,HttpResponse};
use crate::controller::user_handler::{get_user_count_today,get_user_count_this_month,get_user_count_this_year};
use serde::Serialize;

pub async fn track_total_sale(pool: Data<PgPool>) -> impl Responder {
    let result: Result<Option<f64>, SqlxError> = sqlx::query_scalar("SELECT SUM(total_amount::FLOAT8) AS total_amount FROM orders
    ")
        .fetch_one(pool.get_ref()) // Ensure you're passing a reference to the pool
        .await;

    match result {
        Ok(Some(total_sale)) => Ok(HttpResponse::Ok().json(total_sale)), // Correctly handle Option<f64>
        Ok(None) => Ok(HttpResponse::Ok().json(0.0)), // Handle case where no sales are found
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)), // Convert sqlx::Error to actix_web::Error
    }
}

pub async fn total_revenue_till_now(pool: Data<PgPool>) ->impl Responder{
    let result: Result<Option<f64>, SqlxError> = sqlx::query_scalar("SELECT SUM(total_amount::FLOAT8) AS total_amount FROM orders")
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(Some(total_revenue)) => Ok(HttpResponse::Ok().json(total_revenue)),
        Ok(None) => Ok(HttpResponse::Ok().json(0.0)),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn total_yearly_revenue(pool: Data<PgPool>, year: actix_web::web::Path<i32>) -> impl Responder{
    let yr:i32=year.into_inner();
    let result: Result<Option<f64>, SqlxError> = sqlx::query_scalar(
        "SELECT SUM(total_amount::FLOAT8) AS total_amount FROM orders WHERE EXTRACT(YEAR FROM TO_DATE(order_date, 'YYYY-MM-DD')) = $1"
    )
    .bind(yr)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(Some(yearly_revenue)) => Ok(HttpResponse::Ok().json(yearly_revenue)),
        Ok(None) => Ok(HttpResponse::Ok().json(0.0)),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn total_orders_till_now(pool: Data<PgPool>) -> impl Responder {
    let result: Result<Option<i64>, SqlxError> = sqlx::query_scalar("SELECT COUNT(*) FROM orders")
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(Some(total_orders)) => Ok(HttpResponse::Ok().json(total_orders)),
        Ok(None) => Ok(HttpResponse::Ok().json(0)),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}


pub async fn total_monthly_revenue(pool: Data<PgPool>, month: web::Path<i32>) -> impl Responder {
    let mon:i32 = month.into_inner();
    let result: Result<Option<f64>, SqlxError> = sqlx::query_scalar(
        "SELECT SUM(total_amount::FLOAT8) AS total_amount FROM orders WHERE EXTRACT(MONTH FROM TO_DATE(order_date, 'YYYY-MM-DD')) = $1"
    )
    .bind(mon)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(Some(monthly_revenue)) => Ok(HttpResponse::Ok().json(monthly_revenue)),
        Ok(None) => Ok(HttpResponse::Ok().json(0.0)),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}


pub async fn total_revenue_today(pool: web::Data<PgPool>) -> impl Responder {
    let result: Result<Option<f64>, SqlxError> = sqlx::query_scalar(
        "SELECT SUM(total_amount::FLOAT8) AS total_amount FROM orders WHERE TO_DATE(order_date, 'YYYY-MM-DD') = CURRENT_DATE"
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(Some(revenue_today)) => Ok(HttpResponse::Ok().json(revenue_today)),
        Ok(None) => Ok(HttpResponse::Ok().json(0.0)),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn total_orders_today(pool: Data<PgPool>) -> impl Responder {
    let result: Result<Option<i64>, SqlxError> = sqlx::query_scalar("SELECT COUNT(*) FROM orders WHERE TO_DATE(order_date, 'YYYY-MM-DD') = CURRENT_DATE")
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(Some(total_orders)) => Ok(HttpResponse::Ok().json(total_orders)),
        Ok(None) => Ok(HttpResponse::Ok().json(0)),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn total_orders_this_month(pool: Data<PgPool>) -> impl Responder {
    let result: Result<Option<i64>, SqlxError> = sqlx::query_scalar("SELECT COUNT(*) FROM orders WHERE EXTRACT(MONTH FROM TO_DATE(order_date, 'YYYY-MM-DD')) = EXTRACT(MONTH FROM CURRENT_DATE) AND EXTRACT(YEAR FROM TO_DATE(order_date, 'YYYY-MM-DD')) = EXTRACT(YEAR FROM CURRENT_DATE)")
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(Some(total_orders)) => Ok(HttpResponse::Ok().json(total_orders)),
        Ok(None) => Ok(HttpResponse::Ok().json(0)),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn total_orders_this_year(pool: Data<PgPool>) -> impl Responder {
    let result: Result<Option<i64>, SqlxError> = sqlx::query_scalar("SELECT COUNT(*) FROM orders WHERE EXTRACT(YEAR FROM TO_DATE(order_date, 'YYYY-MM-DD')) = EXTRACT(YEAR FROM CURRENT_DATE)")
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(Some(total_orders)) => Ok(HttpResponse::Ok().json(total_orders)),
        Ok(None) => Ok(HttpResponse::Ok().json(0)),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}


// #[derive(Serialize)]
// struct Report {
//     total_revenue: f64,
//     total_orders: i64,
//     total_users: i64,
// }

// async fn gather_report_data(pool: &PgPool, filter: &str) -> Result<Report, SqlxError> {
//     let total_revenue = match filter {
//         "Today" => total_revenue_today(pool).await?,
//         "This Month" => total_monthly_revenue(pool).await?,
//         "This Year" => total_yearly_revenue(pool).await?,
//         _ => panic!("Invalid filter"),
//     }.unwrap_or(0.0);

//     let total_orders = match filter {
//         "Today" => total_orders_today(pool).await?,
//         "This Month" => total_orders_this_month(pool).await?,
//         "This Year" => total_orders_this_year(pool).await?,
//         _ => panic!("Invalid filter"),
//     }.unwrap_or(0);

//     let total_users = match filter {
//         "Today" => get_user_count_today(pool).await?,
//         "This Month" => get_user_count_this_month(pool).await?,
//         "This Year" => get_user_count_this_year(pool).await?,
//         _ => panic!("Invalid filter"),
//     }.unwrap_or(0);

//     Ok(Report {
//         total_revenue,
//         total_orders,
//         total_users,
//     })
// }

// pub async fn get_report(pool: web::Data<PgPool>, filter: web::Path<String>) -> impl Responder {
//     let filter = filter.into_inner();
//     let report_result = gather_report_data(pool.as_ref(), &filter).await;
    
//     match report_result {
//         Ok(report) => HttpResponse::Ok().json(report),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }

