use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::model::address::Address ;

// Create Address
pub async fn create_address(
    address_input: web::Json<Address>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let new_address_input = address_input.into_inner();

    let result = sqlx::query(
        "INSERT INTO address (user_id, address_line1, address_line2, city, add_state, postal_code, country)
         VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind(&new_address_input.user_id)
        .bind(&new_address_input.address_line1)
        .bind(&new_address_input.address_line2)
        .bind(&new_address_input.city)
        .bind(&new_address_input.add_state)
        .bind(&new_address_input.postal_code)
        .bind(&new_address_input.country)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(new_address_input),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


// Get all Addresses by User ID
pub async fn get_addresses_by_user_id(
    user_id: web::Path<i32>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let user_id = user_id.into_inner();

    match sqlx::query_as::<_, Address>(
        "SELECT address_id, user_id, address_line1, address_line2, city, add_state, postal_code, country
         FROM address
         WHERE user_id = $1")
        .bind(&user_id)
        .fetch_optional(pool.as_ref())
        .await
    {
        Ok(Some(address)) => HttpResponse::Ok().json(address),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            eprintln!("Error fetching address: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
        ,
    }
}

// Update Address
pub async fn update_address(
    path_params: web::Path<(i32,)>,
    address_input: web::Json<Address>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let (address_id,) = path_params.into_inner();
    let updated_address_input = address_input.into_inner();

    let result = sqlx::query(
        "UPDATE address
         SET address_line1 = $1, address_line2 = $2, city = $3, add_state = $4, postal_code = $5, country = $6
         WHERE address_id = $7")
        .bind(&updated_address_input.address_line1)
        .bind(&updated_address_input.address_line2)
        .bind(&updated_address_input.city)
        .bind(&updated_address_input.add_state)
        .bind(&updated_address_input.postal_code)
        .bind(&updated_address_input.country)
        .bind(&address_id)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(updated_address_input),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Delete Address
pub async fn delete_address(
    path_params: web::Path<(i32,)>,
    pool: web::Data<PgPool>
) -> impl Responder {
    let (address_id,) = path_params.into_inner();

    let result = sqlx::query(
        "DELETE FROM address
         WHERE address_id = $1")
        .bind(&address_id)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
