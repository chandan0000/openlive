use crate::utils::app_response::ErrorResponse;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::http::StatusCode;

const MAX_PASSWORD_LENGTH: usize = 64;

pub fn hash(password: impl Into<String>) -> Result<String, ErrorResponse> {
    let password = password.into();

    if password.is_empty() {
        return Err(ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            "Password cannot be empty",
            "Password cannot be empty",
        ));
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            format!(
                "Password must not be more than {} characters",
                MAX_PASSWORD_LENGTH
            ),
            "Password must not be more than 64 characters",
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| {
            ErrorResponse::new(
                StatusCode::BAD_GATEWAY,
                "Something went wrong",
                "Password hashing failed",
            )
        })?
        .to_string();

    Ok(hashed_password)
}

pub fn password_match(password: &str, hashed_password: &str) -> Result<bool, ErrorResponse> {
    if password.is_empty() {
        return Err(ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            "Password cannot be empty",
            "Password cannot be empty",
        ));
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            "Password too long",
            format!(
                "Password must not be more than {} characters",
                MAX_PASSWORD_LENGTH
            ),
        ));
    }

    let parsed_hash = PasswordHash::new(hashed_password).map_err(|_| {
        ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            "Invalid hash format",
            "Provided password hash is invalid",
        )
    })?;

    let password_matches = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(password_matches)
}
