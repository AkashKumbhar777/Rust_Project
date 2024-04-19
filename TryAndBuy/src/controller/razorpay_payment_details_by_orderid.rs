use actix_web::{web, HttpResponse};
use reqwest::Client;
// use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE };
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RazorpayPaymentResponse {
    pub id: String,
    pub entity: Option<String>,
    pub amount: Option<i32>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub base_amount: Option<i32>,
    pub base_currency: Option<String>,
    pub method: Option<String>,
    pub order_id: Option<String>,
    pub description: Option<String>,
    pub international: Option<bool>,
    pub refund_status: Option<String>,
    pub amount_refunded: Option<i32>,
    pub captured: Option<bool>,
    pub email: Option<String>,
    pub contact: Option<String>,
    pub fee: Option<i32>,
    pub tax: Option<i32>,
    pub notes: Option<serde_json::Value>,
    pub created_at: Option<i64>,
}

pub async fn fetch_payment_by_order_id(order_id: web::Path<String>) -> HttpResponse {
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

    let client = Client::new();

    let response = client.get(&format!("https://api.razorpay.com/v1/orders/{}/payments", order_id)) 
        .headers(headers.clone())
        .send()
        .await
        .map_err(|err| HttpResponse::BadRequest().body(format!("Failed to send request to Razorpay: {}", err)))
        .unwrap(); // Handle errors appropriately


        let response_json: RazorpayPaymentResponse = response.json().await
        .map_err(|err| HttpResponse::BadRequest().body(format!("Failed to parse Razorpay response: {}", err)))
        .unwrap(); // Handle errors appropriately

    HttpResponse::Ok().json(response_json)
    // {
    //     Ok(response) => response,
    //     Err(err) => {
    //         return HttpResponse::BadRequest().body(format!(
    //             "Failed to send request to Razorpay: {}",
    //             err
    //         ))
    //     }
    // };

    // let response_json: RazorpayPaymentResponse = match response.json().await {
    //     Ok(response_json) => response_json,
    //     Err(err) => {
    //         return HttpResponse::BadRequest().body(format!(
    //             "Failed to parse Razorpay response: {}",
    //             err
    //         ))
    //     }
    // };

    // HttpResponse::Ok().json(response_json)
}
