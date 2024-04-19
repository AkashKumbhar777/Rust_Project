// use actix_web::{web, HttpResponse};
// use serde::{Deserialize, Serialize};
// use reqwest::{Client, StatusCode};
// use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE };
// extern crate base64;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct PayRazorpayRefundRequest {
//     pub amount: f64,
//     pub currency: String,
//     // pub payment_id: String,
// }
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct RefundResponse {
//     pub id: String,
//     pub entity: String,
//     pub amount: i64,
//     pub currency: String,
//     pub payment_id: String,
//     pub receipt: Option<String>,
//     pub acquirer_data: AcquirerData,
//     pub created_at: u64,
//     pub batch_id: Option<String>,
//     pub status: String,
//     pub speed_processed: String,
//     pub speed_requested: String,
// }
// pub async fn refund_payment(payment_id: web::Path<String>,refund_request: web::Json<PayRazorpayRefundRequest>) -> HttpResponse {
//     let refund_request = refund_request.into_inner();
//     const API_KEY: &str = "rzp_test_Mkt5dPv1PBdTQ4";
//     const API_SECRET: &str = "mG8LVjtghMlV116Ty1w2Hrd3";
//     let auth_str = format!("{}:{}", API_KEY, API_SECRET);
//     let encoded_auth = base64::encode(auth_str);

//     let mut headers = HeaderMap::new();
//     headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
//     headers.insert(
//         AUTHORIZATION,
//         HeaderValue::from_str(&format!("Basic {}", encoded_auth)).unwrap(),
//     );

//     // Prepare request to Razorpay API to refund a payment
//     let client = Client::new();
//     let response = client.post(&format!("https://api.razorpay.com/v1/payments/{}/refund",payment_id))
//         .headers(headers)
//         .json(&serde_json::json!({
//             "amount": refund_request.amount,
//             "currency": refund_request.currency,
//         }))
//         .send()
//         .await
//         .map_err(|err| HttpResponse::BadRequest().body(format!("Failed to send request to Razorpay: {}", err)))
//         .unwrap(); // Handle errors appropriately

//     // Parse Razorpay payment refund response
//     let status_code = response.status();
//     match status_code {
//         StatusCode::OK => HttpResponse::Ok().body("Payment successfully refunded"),
//         StatusCode::BAD_REQUEST => HttpResponse::BadRequest().body("Failed to refund payment: Bad request"),
//         StatusCode::UNAUTHORIZED => HttpResponse::Unauthorized().body("Failed to refund payment: Unauthorized"),
//         e => { println!("\n\n error : {}\n", e); 
//             HttpResponse::InternalServerError().body("Failed to refund payment: Unknown error")},
//     }
// }


use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::{Client, StatusCode};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE };
extern crate base64;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PayRazorpayRefundRequest {
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefundResponse {
    pub id: String,
    pub entity: String,
    pub amount: i64,
    pub currency: String,
    pub payment_id: String,
    pub receipt: Option<String>,
    pub acquirer_data: AcquirerData,
    pub created_at: u64,
    pub batch_id: Option<String>,
    pub status: String,
    pub speed_processed: String,
    pub speed_requested: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AcquirerData {
    pub arn: Option<String>,
}

pub async fn refund_payment(payment_id: web::Path<String>, refund_request: web::Json<PayRazorpayRefundRequest>) -> HttpResponse {
    let refund_request = refund_request.into_inner();
    const API_KEY: &str = "rzp_test_Mkt5dPv1PBdTQ4";
    const API_SECRET: &str = "mG8LVjtghMlV116Ty1w2Hrd3";
    let auth_str = format!("{}:{}", API_KEY, API_SECRET);
    let encoded_auth = base64::encode(auth_str);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Basic {}", encoded_auth)).unwrap(),
    );

    // Prepare request to Razorpay API to refund a payment
    let client = Client::new();
    let response = client.post(&format!("https://api.razorpay.com/v1/payments/{}/refund", payment_id))
        .headers(headers)
        .json(&serde_json::json!({
            "amount": refund_request.amount,
            "currency": refund_request.currency,
        }))
        .send()
        .await
        .map_err(|err| HttpResponse::BadRequest().body(format!("Failed to send request to Razorpay: {}", err)))
        .unwrap(); // Handle errors appropriately

    // Parse Razorpay payment refund response
    let status_code = response.status();
    match status_code {
        StatusCode::OK => {
            match response.json::<RefundResponse>().await {
                Ok(refund_response) => HttpResponse::Ok().json(refund_response), // Return the refund response
                Err(_) => HttpResponse::InternalServerError().body("Failed to parse refund response"),
            }
        },
        StatusCode::BAD_REQUEST => HttpResponse::BadRequest().body("Failed to refund payment: Bad request"),
        StatusCode::UNAUTHORIZED => HttpResponse::Unauthorized().body("Failed to refund payment: Unauthorized"),
        e => {
            println!("\n\n error : {}\n", e); 
            HttpResponse::InternalServerError().body("Failed to refund payment: Unknown error")
        },
    }
}
