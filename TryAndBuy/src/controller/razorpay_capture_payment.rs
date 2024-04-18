use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE };
use crate::model::user::User; // Assuming you have a User model
use crate::controller::user_handler::get_user_by_id; // Assuming you have a function to get user by ID
extern crate base64;
use base64::{encode, decode};
//  use actix_web::http::StatusCode;
use reqwest::StatusCode;
   

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PayRazorpayCaptureRequestment {
    pub amount: f64,
    pub currency: String,
}

pub async fn capture_payment(payment_id: web::Path<String> , capture_request: web::Json<PayRazorpayCaptureRequestment>) -> HttpResponse {

    let capture_request = capture_request.into_inner();
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

    // Prepare request to Razorpay API to capture a payment
    let client = Client::new();
    let response = client.post(&format!("https://api.razorpay.com/v1/payments/{}/capture", payment_id))
        .headers(headers)
        .json(&serde_json::json!({
            "amount": capture_request.amount,
            "currency": capture_request.currency,
        }))
        .send()
        .await
        .map_err(|err| HttpResponse::BadRequest().body(format!("Failed to send request to Razorpay: {}", err)))
        .unwrap(); // Handle errors appropriately

    // Parse Razorpay payment capture response
    let status_code = response.status();
    match status_code {
        StatusCode::OK => HttpResponse::Ok().body("Payment successfully captured"),
        StatusCode::BAD_REQUEST => HttpResponse::BadRequest().body("Failed to capture payment: Bad request"),
        StatusCode::UNAUTHORIZED => HttpResponse::Unauthorized().body("Failed to capture payment: Unauthorized"),
        e => { println!("\n\n error : {}\n", e); 
            HttpResponse::InternalServerError().body("Failed to capture payment: Unknown error")},
    }
}
