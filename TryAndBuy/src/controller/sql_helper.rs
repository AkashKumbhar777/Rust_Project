use serde_json:: Value;
use sqlx::Error as SerdeError;
use std::collections::HashMap;
use sqlx::{Pool, Postgres, Error};

pub async fn create(
    pool: &Pool<Postgres>,
    table_name: &str,
    data: &HashMap<&str, &str>,
) -> Result<(), Error> {
    let columns: Vec<&str> = data.keys().copied().collect();
    let values: Vec<&str> = data.values().copied().collect();

    let placeholders = (1..=values.len()).map(|n| format!("${}", n)).collect::<Vec<String>>().join(",");

    let query = format!(
        "INSERT INTO {} ({}) VALUES ({});",
        table_name,
        columns.join(","),
        placeholders
    );

    let mut db_query = sqlx::query(&query);

    for value in &values {
        db_query = db_query.bind(*value);
    }

    db_query.execute(pool).await?;

    Ok(())
}

pub async fn update(
    pool: &Pool<Postgres>,
    table_name: &str,
    id: &str,
    data: &HashMap<&str, &str>,
) -> Result<(), Error> {
    let set_values: Vec<String> = data.iter()
        .map(|(key, value)| format!("{} = '{}'", key, value))
        .collect();

    let query = format!(
        "UPDATE {} SET {} WHERE id = '{}';",
        table_name,
        set_values.join(","),
        id,
    );

    sqlx::query(&query)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete(
    pool: &Pool<Postgres>,
    table_name: &str,
    id: &str,
) -> Result<(), Error> {
    let query = format!(
        "DELETE FROM {} WHERE product_id = '{}';",
        table_name,
        id,
    );

    sqlx::query(&query)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get(
    pool: &Pool<Postgres>,
    table_name: &str,
    id: &str,
    columns: &[&str],
) -> Result<Vec<HashMap<String, Value>>, Error> {
    let base_query = format!(
        "SELECT jsonb_agg(t)::TEXT FROM (SELECT {} FROM {} WHERE id = '{}' LIMIT 1) t;",
        columns.join(","),
        table_name,
        id,
    );

    let row: (Option<String>,) = sqlx::query_as(&base_query)
        .fetch_one(pool)
        .await?;

    let value: &str = row.0.as_deref().unwrap_or("[]");

    let data: Result<Vec<HashMap<String, Value>>, SerdeError> = serde_json::from_str(value)
        .map_err(|err| Error::Decode(Box::new(err))); // Convert SerdeError to sqlx::Error

    Ok(data?) // Unwrap the Result and return
}