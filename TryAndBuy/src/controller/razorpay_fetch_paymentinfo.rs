use actix_web::{web, HttpResponse};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
// use serde_json::Value;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Payment {
//     pub id: String,
//     pub entity: String,
//     pub amount: i32,
//     pub currency: String,
//     pub status: String,
//     pub base_amount: Option<i32>,
//     pub base_currency: Option<String>,
//     pub method: String,
//     pub order_id: String,
//     pub description: Option<String>,
//     pub international: Option<bool>,
//     pub refund_status: Option<String>,
//     pub amount_refunded: i32,
//     pub captured: bool,
//     pub email: String,
//     pub contact: String,
//     pub fee: Option<i32>,
//     pub tax: Option<i32>,
//     pub error_code: Option<String>,
//     pub error_description: Option<String>,
//     pub error_source: Option<String>,
//     pub error_step: Option<String>,
//     pub error_reason: Option<String>,
//     pub acquirer_data: Option<AcquirerData>,
//     pub card_id: Option<String>,
//     pub card: Option<Card>,
//     pub invoice_id: Option<String>,
//     pub notes: Option<Value>,
//     pub created_at: i64,
// }

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
    pub error_code: Option<String>,
    pub error_description: Option<String>,
    pub error_source: Option<String>,
    pub error_step: Option<String>,
    pub error_reason: Option<String>,
    pub acquirer_data: Option<AcquirerData>,
    pub card_id: Option<String>,
    pub card: Option<Card>,
    pub invoice_id: Option<String>,
    pub notes: Option<serde_json::Value>,
    pub created_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AcquirerData {
    // Define fields for acquirer data if needed
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    // Define fields for card data if needed
}

pub async fn fetch_payment(payment_id: web::Path<String>) -> HttpResponse {
    const API_KEY: &str = "rzp_test_Mkt5dPv1PBdTQ4";
    const API_SECRET: &str = "mG8LVjtghMlV116Ty1w2Hrd3";

    let auth_str = format!("{}:{}", API_KEY, API_SECRET);
    let encoded_auth = base64::encode(auth_str);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Basic {}", encoded_auth)).unwrap(),
    );

    println!("\n\n\n\n Headers {:?}\n\n", headers);

    let client = Client::new();
    // let response = client.get(&format!("https://api.razorpay.com/v1/payments/{}", payment_id))
    //     .headers(headers.clone())
    //     .send()
    //     .await
    //     .map_err(|err| HttpResponse::BadRequest().body(format!("Failed to send request to Razorpay: {}", err)))
    //     .unwrap();
    let response = match client
    .get(&format!("https://api.razorpay.com/v1/payments/{}", payment_id))
    .headers(headers.clone())
    .send()
    .await
{
    Ok(response) => {
        println!("\nReceived response: {:?}\n\n\n", response);
        response},
    Err(err) => {
        println!("\nFailed to send request to Razorpay: {:?}\n\n\n", err);
        return HttpResponse::BadRequest().body(format!(
            "Failed to send request to Razorpay: {}",
            err
        ))
    }
};

let response_json: RazorpayPaymentResponse = match response.json().await {
    Ok(response_json) => response_json,
    Err(err) => {
        return HttpResponse::BadRequest().body(format!(
            "Failed to parse Razorpay response: {}",
            err
        ))
    }
};

HttpResponse::Ok().json(response_json)
}