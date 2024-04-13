use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use actix_web::error::InternalError;

#[derive(Deserialize)]
pub struct AuthCode {
    code: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
}

pub async fn authenticate(auth_code: web::Json<AuthCode>) -> Result<HttpResponse, Error> {
    let auth_code = auth_code.into_inner().code;
    println!("Received auth code: {}", auth_code);
    let token = request_token(&auth_code).await?;
    Ok(HttpResponse::Ok().json(token))
}

pub async fn request_token(auth_code: &str) -> Result<TokenResponse, Error> {
    let client = Client::new();
    let response = client.post("https://AKASHPK.b2clogin.com/AKASHPK.onmicrosoft.com/B2C_1_SignUpSignIn/oauth2/v2.0/token")
        .form(&[
            ("grant_type", "authorization_code"),
            ("client_id", "1aaf2355-35a8-428c-9bb6-064079ee6c40"),
            // ("client_secret", "KON8Q~2TvPA6YrvaroOwMUwd2lXLeky3n2~xIa_V"),
            ("code", auth_code),
            ("redirect_uri", "http://localhost:4200/"),
            ("scope", "https://AKASHPK.onmicrosoft.com/api/Try&Buy.read offline_access openid")
        ])
        .send()
        .await
        .map_err(|e| InternalError::new(format!("Failed to send request: {}", e), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR))?;
        println!("{}",response.status());

    if response.status().is_success() {
        let token_response = response.json::<TokenResponse>()
            .await
            .map_err(|e| InternalError::new(format!("Failed to deserialize token response: {}", e), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(token_response)
    } else {
        let status_code = response.status();
        let error_message = response.text().await.unwrap_or_else(|_| "Failed to retrieve error message".to_string());
        Err(InternalError::new(format!("Token request failed with status {}: {}", status_code, error_message), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR).into())
    }
}


