use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::model::user::User; // Assuming the User struct is defined in the same module as Product

pub async fn create_user(user_input: web::Json<User>, pool: web::Data<PgPool>) -> impl Responder {
    let new_user_input = user_input.into_inner();

    let result = sqlx::query(
        "INSERT INTO user_table (first_name, last_name, email, phone, profile_picture, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind(&new_user_input.first_name)
        .bind(&new_user_input.last_name)
        .bind(&new_user_input.email)
        .bind(&new_user_input.phone)
        .bind(&new_user_input.profile_picture)
        .bind(&new_user_input.created_at)
        .bind(&new_user_input.updated_at)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(new_user_input),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT user_id, login_id, first_name, last_name, email, phone, profile_picture, created_at, updated_at FROM user_table")
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
        "SELECT user_id, login_id, first_name, last_name, email, phone, profile_picture, created_at, updated_at
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

