use crate::utils::app_response::ErrorResponse;
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize}; 

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn create_token(user_id: &str) -> Result<String, ErrorResponse> {
    if user_id.is_empty() {
        return Err(ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            "Invalid subject",
            "User ID cannot be empty",
        ));
    }
    let secret = dotenv!("JWT_SECRET");
    let duration_insecond: i64 = dotenv!("JWT_DURATION")
        .parse::<i64>()
        .expect("JWT_DURATION must be a valid integer");
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::seconds(duration_insecond)).timestamp() as usize;

    let claims: TokenClaims = TokenClaims {
        sub: user_id.to_string(),
        iat,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| {
        ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Token creation failed",
            "Could not generate token",
        )
    })
}

pub fn decode_token<T: Into<String>>(token: T) -> Result<String, ErrorResponse> {
    let secret = dotenv!("JWT_SECRET");
    let decoded = decode::<TokenClaims>(
        &token.into(),
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    );

    match decoded {
        Ok(token) => Ok(token.claims.sub),
        Err(_) => Err(ErrorResponse::new(
            StatusCode::UNAUTHORIZED,
            "Invalid token",
            "Token verification failed",
        )),
    }
}
