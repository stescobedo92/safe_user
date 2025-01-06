use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::{BearerAuth};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation, Header, encode, decode};
use chrono::{Utc, Duration};
use serde::{Deserialize, Serialize};
use std::env;

/// This module provides JWT generation and validation functionalities.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

/// Generates a JWT for the given subject.
///
/// # Arguments
///
/// * `sub` - A string slice that holds the subject for which the JWT is generated.
///
/// # Returns
///
/// * `Result<String, jsonwebtoken::errors::Error>` - A result containing the generated JWT as a string or an error.
pub fn generate_jwt(sub: &String) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
    let expiration = Utc::now() + Duration::hours(24);

    let claims = Claims {
        sub: sub.to_owned(),
        exp: expiration.timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

/// Validates a given JWT and returns the claims if the token is valid.
///
/// # Arguments
///
/// * `token` - A string slice that holds the JWT to be validated.
///
/// # Returns
///
/// * `Result<Claims, jsonwebtoken::errors::Error>` - A result containing the claims if the token is valid or an error.
pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
    let validation = Validation::default();

    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation)?;
    Ok(token_data.claims)
}

/// Middleware function to validate JWT in incoming requests.
///
/// # Arguments
///
/// * `req` - The incoming service request.
/// * `credentials` - The bearer authentication credentials extracted from the request.
///
/// # Returns
///
/// * `Result<ServiceRequest, (Error, ServiceRequest)>` - A result containing the service request if the token is valid, or an error and the service request if the token is invalid.
pub async fn jwt_validator(req: ServiceRequest,credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();

    match validate_jwt(token) {
        Ok(_claims) => {
            Ok(req)
        }
        Err(_) => {
            Err((actix_web::error::ErrorUnauthorized("Invalid token"), req))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_jwt() {
        //Check that it doesn't fail and generate a token
        let token = generate_jwt(&"tester".to_string()).expect("Failed to generate JWT");
        assert!(!token.is_empty(), "Token should not be empty");
    }

    #[test]
    fn test_validate_jwt_valido() {
        let token = generate_jwt(&"tester".to_string()).unwrap();
        let claims = validate_jwt(&token).expect("Failed to validate JWT");
        assert_eq!(claims.sub, "tester");
    }

    #[test]
    fn test_validate_jwt_invalido() {
        // A completely invalid token
        let result = validate_jwt("non-existent_token");
        assert!(result.is_err(), "Validation of invalid token should fail");
    }
}