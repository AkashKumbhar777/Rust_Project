use actix_web::{web,HttpResponse,Responder};
use sqlx::{prelude::FromRow, PgPool};
use crate::model::order::Order;
use serde::{Serialize,Deserialize};



#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrderWithDetails {
    pub order_id: i32,
    pub user_id: i32,
    pub user_firstname: String,
    pub user_lastname: String,
    pub product_name: String,
    pub order_status: String,
    pub order_date: String,
    pub total_amount: f64,
}

pub async fn create_order(order : web::Json<Order> , pool : web::Data<PgPool>) -> impl Responder {
    let new_order = order.into_inner()  ;
    let result = sqlx::query("Insert into orders(user_id,product_id,order_status,order_date,total_amount)
    VALUES($1,$2,$3,$4,$5)")
    .bind(&new_order.user_id)
    .bind(&new_order.product_id)
    .bind(&new_order.order_status)
    .bind(&new_order.order_date)
    .bind(&new_order.total_amount)
    // .bind(&new_order.address_id)
    .execute(pool.as_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(new_order),
        Err(err) =>{
                        println!("Failed to create order: {}", err);
                         HttpResponse::InternalServerError().finish()
                    },
                 }
 }

pub async fn get_orders(pool : web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_,Order>("Select * from orders")
    .fetch_all(pool.as_ref())
    .await ; 

    match result { 
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(err) => {
            println!("Failed to get orders: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async  fn get_orders_by_userid(user_id:web::Path<i32> , pool: web::Data<PgPool>)->impl Responder{
    let user_id = user_id.into_inner();
    let result  = sqlx::query_as::<_,Order>("Select * from orders where user_id =$1")
    .bind(&user_id)
    .fetch_all(pool.as_ref())
    .await ;

    match result{
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(err) => {
            println!("Failed to get orders: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}


pub async fn update_order(order_input:web::Json<Order> , order_id : web::Path<i32>, pool : web::Data<PgPool>) -> impl Responder{
    let updated_order = order_input.into_inner() ; 
    let orders_id = order_id.into_inner() ;
    let result  = sqlx::query("Update orders set user_id =$1,
                           product_id =$2,
                           order_status =$3,
                           order_date =$4,
                           total_amount =$5 where order_id =$6")
    .bind(&updated_order.user_id)
    .bind(&updated_order.product_id)
    .bind(&updated_order.order_status)
    .bind(&updated_order.order_date)
    .bind(&updated_order.total_amount)
    .bind(&orders_id)
    .execute(pool.as_ref())
    .await ;

    match result{
        Ok(_) => HttpResponse::Ok().json(updated_order),
        Err(err) => {
            println!("Failed to update order: {}", err);
            HttpResponse::InternalServerError().finish()
        }
 }
}

pub async fn delete_order(order_id :web::Path<i32> , pool :web::Data<PgPool>) -> impl Responder {
    let order_id =  order_id.into_inner();
    let result = sqlx::query("Delete from orders where order_id = $1")
    .bind(&order_id)
    .execute(pool.as_ref())
    .await;

    match result 
    {
        Ok(_)=> HttpResponse::Ok().finish(),
        Err(err) => {
            println!("Failed to delete order: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
   
}

pub async fn get_orders_with_details(pool: web::Data<PgPool>) -> impl Responder {
    // Write the SQL query to fetch orders with user and product details using JOIN
    let query = "
    SELECT 
        o.order_id,
        o.user_id,
        u.first_name AS user_firstname,
        u.last_name AS user_lastname,
        p.product_name,
        o.order_status,
        o.order_date,
        o.total_amount::FLOAT8 as total_amount
    FROM 
        orders o
    JOIN 
        user_table u ON o.user_id = u.user_id
    JOIN 
        product p ON o.product_id = p.product_id;
    ";

    // Execute the query and fetch results
    match sqlx::query_as::<_, OrderWithDetails>(query) // OrderWithDetails should be a struct representing the combined order details
        .fetch_all(pool.as_ref())
        .await
    {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(err) => {
            println!("Failed to get orders with details: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
