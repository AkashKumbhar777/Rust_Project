use actix_web::{web, HttpResponse, Error, error::InternalError};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use crate::model::user::User;
use crate::db_connect::db::get_pool;
use base64::decode as base64_decode;


#[derive(Deserialize)]
pub struct AuthCode {
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    extension_first_name: String,
    extension_last_name: String,
    emails: Vec<String>,
    extension_phone: String,
    extension_created_at: String,
}

pub async fn authenticate(auth_code: web::Json<AuthCode>) -> Result<HttpResponse, Error> {
    let auth_code = auth_code.into_inner().code;
    println!("Received auth code: {}", auth_code);
    let token = request_token(&auth_code).await?;
    println!("{:?}", token);
    let user_info = fetch_user_info(&token.access_token).await?;
    save_user_info_to_database(&user_info).await?;
    Ok(HttpResponse::Ok().json(token))
}

pub async fn request_token(auth_code: &str) -> Result<TokenResponse, Error> {
    let client = Client::new();
    let response = client.post("https://AKASHPK.b2clogin.com/AKASHPK.onmicrosoft.com/B2C_1_SignUpSignIn/oauth2/v2.0/token")
        .form(&[
            ("grant_type", "authorization_code"),
            ("client_id", "1aaf2355-35a8-428c-9bb6-064079ee6c40"),
            ("client_secret", "KON8Q~2TvPA6YrvaroOwMUwd2lXLeky3n2~xIa_V"),
            ("code", auth_code),
            ("redirect_uri", "http://localhost:4200/"),
            ("scope", "https://AKASHPK.onmicrosoft.com/api/Try&Buy.read offline_access openid")
        ])
        .send()
        .await
        .map_err(|e| InternalError::new(format!("Failed to send request: {}", e), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    println!("{}", response.status());

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

pub async fn fetch_user_info(access_token: &str) -> Result<User, Error> {
    // Decode the access token
    let parts: Vec<&str> = access_token.split('.').collect();
    if parts.len() != 3 {
        return Err(InternalError::new("Invalid access token format".to_string(), actix_web::http::StatusCode::BAD_REQUEST).into());
    }

    // Ensure the length of the base64-encoded string is a multiple of 4
    let mut padded_payload = parts[1].to_owned();
    let padding_needed = 4 - (padded_payload.len() % 4);
    for _ in 0..padding_needed {
        padded_payload.push('=');
    }
    print!("Before Decoding");
    print!("{:?}", padded_payload);
    let payload_bytes = base64_decode(&padded_payload).map_err(|e| InternalError::new(format!("Failed to decode payload: {}", e), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    print!("{:?}",payload_bytes);
    let payload_str = String::from_utf8_lossy(&payload_bytes);
    print!("After Decoding");
    print!("{:?}",payload_str);
    // Parse the payload as JSON
    let token_claims: TokenClaims = serde_json::from_str(&payload_str).map_err(|e| InternalError::new(format!("Failed to parse payload: {}", e), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    // Extract user information from the decoded token claims
    let user_info = User {
        user_id: None,
        first_name: token_claims.extension_first_name,
        last_name: token_claims.extension_last_name,
        email: token_claims.emails,
        phone: token_claims.extension_phone,
        profile_picture: None,
        created_at: token_claims.extension_created_at,
        updated_at: None,
    };

    Ok(user_info)
}



pub async fn save_user_info_to_database(user_info: &User) -> Result<(), Error> {
    let pool = get_pool().await.expect("failed to get pool");

    match sqlx::query!(
        "INSERT INTO user_table (first_name, last_name, email, phone, profile_picture, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        user_info.first_name,
        user_info.last_name,
        &user_info.email,
        user_info.phone,
        user_info.profile_picture,
        user_info.created_at,
        user_info.updated_at,
    )
    .execute(&pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(InternalError::new(err, actix_web::http::StatusCode::INTERNAL_SERVER_ERROR).into()),
    }
}
