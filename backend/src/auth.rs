use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::entities::user;
use axum::{
    async_trait,
    extract::{FromRequest, Extension},
    http::{StatusCode, Request},
    response::IntoResponse,
};
use serde_json::json;
use crate::entities::user::Model as UserModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,           // User ID
    pub profile_id: String,    // Profile ID
    pub directory_id: String,  // Directory ID
    pub exp: usize,            // Expiration time
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn generate_jwt(claims: Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header::default();
    let encoding_key = EncodingKey::from_secret("your-secret-key".as_ref());

    encode(&header, &claims, &encoding_key)
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret("your-secret-key".as_ref());
    let validation = Validation::default();

    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}

pub struct AuthenticatedUser(pub UserModel);

#[async_trait]
impl<B, S> FromRequest<S, B> for AuthenticatedUser
where
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request(
        req: Request<B>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Example: Extract token and verify
        // Here you should implement your actual authentication logic
        // For demonstration, we'll assume the user is always authenticated
        // and provide a dummy user

        // Attempt to extract the user from extensions
        if let Some(user) = req.extensions().get::<UserModel>().cloned() {
            Ok(AuthenticatedUser(user))
        } else {
            Err((
                StatusCode::UNAUTHORIZED,
                axum::Json(json!({ "error": "Unauthorized" })),
            ))
        }
    }
}