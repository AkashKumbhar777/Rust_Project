use serde_json:: Value;
use sqlx::Error as SerdeError;
use std::collections::HashMap;
use sqlx::{Pool, Postgres, Error};
use std::any::Any;
use std::fmt::Debug;
use std::fmt;

// pub struct DynamicValue {
//     value: Box<dyn Any>,
//     sql_type: &'static str,
// }

// impl DynamicValue {
//     // Helper method to create a new dynamic value
//    pub fn new<T: 'static + Any>(value: T, sql_type: &'static str) -> Self {
//         DynamicValue {
//             value: Box::new(value),
//             sql_type,
//         }
//     }

//     // Get the value as a reference to `Any`
//    pub fn as_any(&self) -> &dyn Any {
//         &*self.value
//     }
// }

// // Implement Debug for DynamicValue for better error messages
// impl fmt::Debug for DynamicValue {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "DynamicValue({:?})", self.value)
//     }
// }

// // Function to create a new record in the database
// pub async fn create(
//     pool: &sqlx::Pool<sqlx::Postgres>,
//     table_name: &str,
//     data: &HashMap<&str, DynamicValue>,
// ) -> Result<(), sqlx::Error> {
//     print!("{:?}",data);
//     let columns: Vec<&str> = data.keys().copied().collect();
//     let placeholders = (1..=columns.len()).map(|n| format!("${}", n)).collect::<Vec<String>>().join(",");

//     let query = format!(
//         "INSERT INTO {} ({}) VALUES ({});",
//         table_name,
//         columns.join(","),
//         placeholders
//     );

//     let mut db_query = sqlx::query(&query);

//     for key in &columns {
//         if let Some(value) = data.get(*key) {
//             // Convert the dynamic value to the appropriate SQLx type
//             match value.sql_type {
//                 "text" => {
//                     let value = value.as_any().downcast_ref::<String>().unwrap();
//                     db_query = db_query.bind(value);
//                 }
//                 "integer" => {
//                     let value = value.as_any().downcast_ref::<i32>().unwrap();
//                     db_query = db_query.bind(value);
//                 }
//                 // Add more cases for other types as needed
//                 _ => return Err(sqlx::Error::Protocol(format!("Unsupported SQL type: {}", value.sql_type))),
//             }
//         } else {
//             return Err(sqlx::Error::ColumnNotFound(format!("Column '{}' not found in data", *key)));
//         }
//     }

//     db_query.execute(pool).await?;

//     Ok(())
// }

// pub async fn update(
//     pool: &Pool<Postgres>,
//     table_name: &str,
//     id: &str,
//     data: &HashMap<&str, &str>,
// ) -> Result<(), Error> {
//     let set_values: Vec<String> = data.iter()
//         .map(|(key, value)| format!("{} = '{}'", key, value))
//         .collect();

//     let query = format!(
//         "UPDATE {} SET {} WHERE id = '{}';",
//         table_name,
//         set_values.join(","),
//         id,
//     );

//     sqlx::query(&query)
//         .execute(pool)
//         .await?;

//     Ok(())
// }

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
        .map_err(|err| Error::Decode(Box::new(err)));

    Ok(data?) // Unwrap the Result and return
}