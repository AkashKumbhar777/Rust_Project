use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE };
use crate::model::user::User; // Assuming you have a User model
use crate::controller::user_handler::get_user_by_id; // Assuming you have a function to get user by ID
extern crate base64;
use base64::{encode, decode};

#[derive(Debug, Serialize, Deserialize)]
pub struct RazorpayOrderRequest {
    amount: u64,
    currency: String,
    // Add more fields as needed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RazorpayOrderResponse {
    id: String,
    amount: u64,
    currency: String,
    entity: String,
    receipt: Option<String>,
    status: String,
    attempts: u32,
    notes: Option<serde_json::Value>,
    created_at: u64,
    // Add more fields as needed
}

pub async fn create_order(order_request: web::Json<RazorpayOrderRequest>) -> HttpResponse {
    let order_request = order_request.into_inner();

            const API_KEY: &str = "rzp_test_y39gdD0Y9KbAMu";
            const API_SECRET: &str = "RCfAdxOPzBoQDPrgj6gayevq";

            let auth_str = format!("{}:{}", API_KEY, API_SECRET);
            let encoded_auth = base64::encode(auth_str);

            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Basic {}", encoded_auth)).unwrap(),
            );

            // Prepare request to Razorpay API to create an order
            let client = Client::new();
            let response = client.post("https://api.razorpay.com/v1/orders")
                .headers(headers)
                .json(&serde_json::json!({
                    "amount": order_request.amount,
                    "currency": order_request.currency,
                    "receipt": "receipt_id_1",
                    "payment_capture": true,
                    "notes": {
                        "name": "tushar",
                        "email": "jfhvjfw",
                        "contact": "3234242423423"
                    }
                }))
                .send()
                .await
                .map_err(|err| HttpResponse::BadRequest().body(format!("Failed to send request to Razorpay: {}", err)))
                .unwrap(); // Handle errors appropriately

            // Parse Razorpay order creation response
            let response_json: RazorpayOrderResponse = response.json().await
                .map_err(|err| HttpResponse::BadRequest().body(format!("Failed to parse Razorpay response: {}", err)))
                .unwrap(); // Handle errors appropriately

            HttpResponse::Ok().json(response_json)
        
       
            }
    

