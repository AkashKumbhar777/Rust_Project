use actix_web::{web,HttpRequest, HttpResponse, Error, error::InternalError, HttpResponseBuilder, HttpMessage,http::StatusCode};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use crate::model::user::User;
use crate::db_connect::db::get_pool;
use base64::decode as base64_decode;
use actix_web::web::Data;
use std::sync::Arc;

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

//Authentication main method for login and Signup
pub async fn authenticate(auth_code: web::Json<AuthCode>) -> Result<HttpResponse, Error> {
    let auth_code = auth_code.into_inner().code;
    println!("Received auth code: {}", auth_code);
    let token = request_token(&auth_code).await?;
    println!("{:?}", token);
    let user_info = fetch_user_info(&token.access_token).await?;
    save_user_info_to_database(&user_info).await?;
    let token_json = serde_json::to_string(&token)
        .map_err(|e| InternalError::new(format!("Failed to serialize token: {}", e), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok()
        .header("Content-Type", "application/json")
        .header("Location", "http://your_redirect_uri")
        .body(token_json))
}

//Getting JSON Token by using this method
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

//Decoding the Json token and fetching User Information
pub async fn fetch_user_info(access_token: &str) -> Result<User, Error> {
    // Decoding the access token
    let parts: Vec<&str> = access_token.split('.').collect();
    if parts.len() != 3 {
        return Err(InternalError::new("Invalid access token format".to_string(), actix_web::http::StatusCode::BAD_REQUEST).into());
    }

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
    // Parsing the payload as JSON
    let token_claims: TokenClaims = serde_json::from_str(&payload_str).map_err(|e| InternalError::new(format!("Failed to parse payload: {}", e), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    // Extract user information from the decoded token claims
    let user_info = User {
        user_id: None,
        first_name: token_claims.extension_first_name,
        last_name: token_claims.extension_last_name,
        email: token_claims.emails,
        phone: token_claims.extension_phone,
        profile_picture: None,
        user_role: None,
        created_at: token_claims.extension_created_at,
        updated_at: None,
    };

    Ok(user_info)
}


//After Decoding the token UserInfo Saved in database if new user else while logIn user exist and return User.
pub async fn save_user_info_to_database(user_info: &User) -> Result<User, Error> {
    let pool = get_pool().await.expect("failed to get pool");
    let emailid = &user_info.email[0];
    
    // Check if user already exists in the database by email
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT user_id, first_name, last_name, email, phone, profile_picture,user_role, created_at, updated_at
         FROM user_table
         WHERE $1 = ANY(email)")
       .bind(emailid) 
       .fetch_optional(&pool)
       .await
       .map_err(|e| InternalError::new(format!("Failed to check existing user: {}", e), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    let user: Result<User, Error> = match existing_user {
        Some(user) => Ok(user),
        None => Err(InternalError::new("User not found", actix_web::http::StatusCode::NOT_FOUND).into()),
    };

    if let Ok(user) = user {
        print!("{:?}", user);
        return Ok(user);
    }

    // If user does not exist, insert new user into the database
    let inserted_user = sqlx::query!(
        "INSERT INTO user_table (first_name, last_name, email, phone, profile_picture, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        user_info.first_name,
        user_info.last_name,
        &user_info.email,
        user_info.phone,
        user_info.profile_picture,
        user_info.created_at,
        user_info.updated_at,
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| InternalError::new(format!("Failed to insert user: {}", e), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR))?;

    let user = User {
        user_id: Some(inserted_user.user_id),
        first_name: inserted_user.first_name.unwrap_or_default(),
        last_name: inserted_user.last_name.unwrap_or_default(),
        email: inserted_user.email.unwrap_or_default(),
        phone: inserted_user.phone.unwrap_or_default(),
        profile_picture: inserted_user.profile_picture,
        user_role: inserted_user.user_role,
        created_at: inserted_user.created_at.unwrap_or_default(),
        updated_at: inserted_user.updated_at,
    };
    
    Ok(user)
}



// pub async fn authentication_middleware(
//     req: HttpRequest,
//     srv: ServiceRequest,
//     payload: &mut actix_web::dev::Payload,
// ) -> Result<ServiceResponse, Error> {
//     // Extract the JWT token from the request headers
//     let authorization_header = req.headers().get("Authorization");
//     let jwt_token = match authorization_header {
//         Some(header_value) => {
//             match header_value.to_str() {
//                 Ok(token_str) => {
//                     if token_str.starts_with("Bearer ") {
//                         Some(token_str.trim_start_matches("Bearer ").to_owned())
//                     } else {
//                         None
//                     }
//                 },
//                 Err(_) => None,
//             }
//         },
//         None => None,
//     };

//     // Validate the JWT token and fetch user information
//     let user_info = match jwt_token {
//         Some(token) => fetch_user_info(&token).await,
//         None => Err(actix_web::error::InternalError::new("No JWT token provided", actix_web::http::StatusCode::UNAUTHORIZED).into()),
//     };

//     // If user info is successfully obtained, attach it to the request for use in route handlers
//     let user_info = match user_info {
//         Ok(user_info) => user_info,
//         Err(_) => {
//             return Ok(srv.error_response(
//     HttpResponse::Unauthorized().body("Unauthorized").into()
// ));
//         }
//     };
    
//     // Attach user information to the request for use in route handlers
//     let req = req.extensions_mut().insert(user_info);

//     // Proceed with the request
//     let srv = Arc::new(srv); // Convert srv to Arc for cloning
//     let response = srv.call(req, payload).await?;

//     Ok(response)
// }