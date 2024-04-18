use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
// use crate::model::user::User; // Assuming you have a User model
// use crate::controller::user_handler::get_user_by_id; // Assuming you have a function to get user by ID
extern crate base64;
// use base64::{encode, decode};
use reqwest::StatusCode;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RazorpayCreatePaymentRequest {
    pub amount: f64,
    pub currency: String,
    pub user_id: String, // Assuming you want to associate payments with users
    // Add any other necessary fields here
}

pub async fn create_payment(create_request: web::Json<RazorpayCreatePaymentRequest>) -> HttpResponse {
    let create_request = create_request.into_inner();

    const API_KEY: &str = "YOUR_RAZORPAY_API_KEY";
    const API_SECRET: &str = "YOUR_RAZORPAY_API_SECRET";

    let auth_str = format!("{}:{}", API_KEY, API_SECRET);
    let encoded_auth = base64::encode(auth_str);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Basic {}", encoded_auth)).unwrap(),
    );

    // Prepare request to Razorpay API to create a payment
    let client = Client::new();
    let response = client.post("https://api.razorpay.com/v1/payments")
        .headers(headers)
        .json(&serde_json::json!({
            "amount": create_request.amount,
            "currency": create_request.currency,
            "notes": {
                "user_id": create_request.user_id,
                // Add any other necessary fields here
            }
        }))
        .send()
        .await
        .map_err(|err| HttpResponse::BadRequest().body(format!("Failed to send request to Razorpay: {}", err)))
        .unwrap(); // Handle errors appropriately

    // Parse Razorpay payment creation response
    let status_code = response.status();
    match status_code {
        StatusCode::CREATED => {
            let payment_response: serde_json::Value = response.json().await.unwrap(); // You may want to handle errors here
            let payment_id = payment_response["id"].as_str().unwrap(); // Extract payment ID
            HttpResponse::Created().body(format!("Payment created with ID: {}", payment_id))
        },
        StatusCode::BAD_REQUEST => HttpResponse::BadRequest().body("Failed to create payment: Bad request"),
        StatusCode::UNAUTHORIZED => HttpResponse::Unauthorized().body("Failed to create payment: Unauthorized"),
        e => {
            println!("\n\n error : {}\n", e);
            HttpResponse::InternalServerError().body("Failed to create payment: Unknown error")
        },
    }
}
