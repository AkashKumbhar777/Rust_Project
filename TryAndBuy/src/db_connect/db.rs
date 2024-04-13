use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Error;


pub async fn get_pool() -> Result<PgPool, Error> {
    // Create a database pool
    print!("inside db pool");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://postgres:Akash777*@localhost/TryAndBuy2")
        .await?;

    Ok(pool)
}