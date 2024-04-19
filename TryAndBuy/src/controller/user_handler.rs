use actix_web::{web, HttpResponse, Responder ,error::ErrorInternalServerError};
use sqlx::PgPool;
use crate::model::user::User; 
use std::io::Write;


pub async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT user_id, first_name, last_name, email, phone, profile_picture,user_role, created_at, updated_at FROM user_table")
        .fetch_all(pool.as_ref())
        .await
    {
        Ok(users) => {
            println!("Retrieved {} users", users.len());
            HttpResponse::Ok().json(users)
        },
        Err(err) => {
            println!("Failed to retrieve users: {}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}
pub async fn get_user_by_id(
    user_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let user_id = user_id.into_inner();

    match sqlx::query_as::<_, User>(
        "SELECT user_id, first_name, last_name, email, phone, profile_picture,user_role, created_at, updated_at
         FROM user_table
         WHERE user_id = $1")
        .bind(&user_id)
        .fetch_optional(pool.as_ref())
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_user_by_email(
    email: web::Path<String>, // Assuming email is a String type
    pool: web::Data<PgPool>
) -> impl Responder {
    let email = email.into_inner();

    match sqlx::query_as::<_, User>(
        "SELECT user_id, first_name, last_name, email, phone, profile_picture, created_at, updated_at,user_role
         FROM user_table
         WHERE email = $1")
        .bind(&email)
        .fetch_optional(pool.as_ref())
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {print!("e={:?}",e);
            HttpResponse::InternalServerError().finish()
        },
    }
}



pub async fn update_user(
    user_id: web::Path<i32>,
    user_input: web::Json<User>,
    pool: web::Data<PgPool>
) -> impl Responder {

    let user_id = user_id.into_inner();
    let updated_user_input = user_input.into_inner();
    println!("{:?}", updated_user_input);

    let result = sqlx::query(
        "UPDATE user_table SET
         first_name = $1,
         last_name = $2,
         email = $3,
         phone = $4,
         profile_picture = $5,
         updated_at = $6
         WHERE user_id = $7")
        // .bind(&updated_user_input.login_id)
        .bind(&updated_user_input.first_name)
        .bind(&updated_user_input.last_name)
        .bind(&updated_user_input.email)
        .bind(&updated_user_input.phone)
        .bind(&updated_user_input.profile_picture)
        .bind(&updated_user_input.updated_at)
        .bind(&user_id)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(updated_user_input),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_user(user_id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {

    let user_id = user_id.into_inner();
    let result = sqlx::query(
        "DELETE FROM user_table WHERE user_id = $1")
        .bind(&user_id)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_user_count_this_month(pool: web::Data<PgPool>) -> impl Responder {
    let result: Result<Option<i64>, sqlx::Error> = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM user_table
         WHERE EXTRACT(YEAR FROM CURRENT_DATE) = EXTRACT(YEAR FROM TO_DATE(created_at, 'YYYY-MM-DD'))
         AND EXTRACT(MONTH FROM CURRENT_DATE) = EXTRACT(MONTH FROM TO_DATE(created_at, 'YYYY-MM-DD'))"
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(Some(user_count)) => HttpResponse::Ok().json(user_count),
        Ok(None) => HttpResponse::Ok().json(0),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_user_count_this_year(pool: web::Data<PgPool>) -> impl Responder {
    let result: Result<Option<i64>, sqlx::Error> = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM user_table
         WHERE EXTRACT(YEAR FROM CURRENT_DATE) = EXTRACT(YEAR FROM TO_DATE(created_at, 'YYYY-MM-DD'))"
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(Some(user_count)) => HttpResponse::Ok().json(user_count),
        Ok(None) => HttpResponse::Ok().json(0),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_user_count_today(pool: web::Data<PgPool>) -> impl Responder {
    let result: Result<Option<i64>, sqlx::Error> = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM user_table
         WHERE CURRENT_DATE = TO_DATE(created_at, 'YYYY-MM-DD')"
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(Some(user_count)) => HttpResponse::Ok().json(user_count),
        Ok(None) => HttpResponse::Ok().json(0),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_profile_picture(
    user_id: web::Path<i32>,
    file: web::Bytes,
    pool: web::Data<PgPool>
) -> Result<impl Responder, actix_web::Error> {

    let user_id = user_id.into_inner();

    let picture_data: Vec<u8> = file.into_iter().collect();


    // Convert web::Bytes into a Vec<u8>
    // let mut picture_data: Vec<u8> = Vec::new();
    // if let Err(err) = picture_data.write_all(&profile_picture) {
    //     return Err(ErrorInternalServerError(format!("Failed to write profile picture data: {}", err)));
    // }

    // Prepare the query to update the profile picture
    let result = sqlx::query(
        "UPDATE user_table
         SET profile_picture = $1
         WHERE user_id = $2")
        .bind(&picture_data)
        .bind(&user_id)
        .execute(pool.as_ref())
        .await;

    // Return appropriate response based on the query result
    match result {
        Ok(_) => {
            println!("Profile pic updated successfully");
            Ok(HttpResponse::Ok().finish())},
        Err(_) => Err(ErrorInternalServerError("Failed to update profile picture")),
    }
}
