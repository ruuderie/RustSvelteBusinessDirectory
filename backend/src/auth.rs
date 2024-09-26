use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use chrono::{Utc, Duration};
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::entities::{profile, user};
use axum::{
    async_trait,
    extract::{FromRequest, Extension},
    http::{StatusCode, Request},
    response::IntoResponse,
};
use serde_json::json;
use crate::entities::user::Model as UserModel;
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,           // User ID
    pub profile_id: String,    // Profile ID
    pub directory_id: String,  // Directory ID
    pub is_admin: bool,        // Admin flag
    pub exp: usize,            // Expiration time
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimsAdmin {
    pub sub: String,           // User ID
    pub is_admin: bool,        // Admin flag
    pub exp: usize,            // Expiration time
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    tracing::debug!("Verifying password");
    verify(password, hash)
}

pub fn generate_jwt(user: &user::Model, profile: &profile::Model) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

        // Generate JWT including user_id, profile_id, and directory_id
        let claims = Claims {
            sub: user.id.to_string(),
            profile_id: profile.id.to_string(),
            directory_id: profile.directory_id.to_string(),
            exp: expiration as usize,
            is_admin: user.is_admin,
        };

    let header = Header::default();
    let encoding_key = EncodingKey::from_secret("your-secret-key".as_ref());

    encode(&header, &claims, &encoding_key)
}
pub fn generate_jwt_admin(user: &user::Model) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = ClaimsAdmin {
        sub: user.id.to_string(),
        exp: expiration as usize,
        is_admin: user.is_admin,
    };

    let header = Header::default();
    let encoding_key = EncodingKey::from_secret("your-secret-key".as_ref());

    encode(&header, &claims, &encoding_key)
}

pub fn validate_jwt<T: DeserializeOwned>(token: &str) -> Result<T, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret("your-secret-key".as_ref());
    let validation = Validation::default();

    tracing::debug!("Attempting to decode JWT");
    tracing::debug!("Token: {}", token);
    tracing::debug!("Validation: {:?}", validation);

    let token_data = decode::<T>(token, &decoding_key, &validation)?;
    tracing::debug!("JWT decoded successfully for {:?}", std::any::type_name::<T>());
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