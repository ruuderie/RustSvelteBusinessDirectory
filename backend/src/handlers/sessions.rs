use axum::{
    extract::{Extension, Json, TypedHeader},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    headers::{Authorization, authorization::Bearer},
};
use sea_orm::{DatabaseConnection, ColumnTrait, EntityTrait, Set, ActiveModelTrait, QueryFilter};
use uuid::Uuid;
use chrono::{Utc, Duration};
use crate::entities::{directory, session, user};
use crate::auth::{generate_jwt, hash_password, verify_password};
use crate::handlers::users::LoginResponse;
use crate::models::user::UserLogin;
use crate::models::session::SessionResponse;

pub async fn create_session(
    Extension(db): Extension<DatabaseConnection>,
    Json(credentials): Json<UserLogin>,
) -> Result<SessionResponse, StatusCode> {
    tracing::info!("Creating session for user: {}", credentials.email);

    let user = match user::Entity::find()
        .filter(user::Column::Email.eq(credentials.email.clone()))
        .one(&db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            tracing::warn!("User not found for email: {}", credentials.email);
            return Err(StatusCode::UNAUTHORIZED);
        }
        Err(e) => {
            tracing::error!("Database error when finding user: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if !verify_password(&credentials.password, &user.password_hash).map_err(|e| {
        tracing::error!("Error verifying password: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })? {
        tracing::warn!("Invalid password for user: {}", user.id);
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = generate_jwt(&user).map_err(|e| {
        tracing::error!("Error generating JWT: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    tracing::info!("Generated token for user {}: {}", user.id, token); // Add this line
    tracing::debug!("Generated token: {}", token);
    let refresh_token = Uuid::new_v4().to_string();

    let new_session = session::Model {
        id: Uuid::new_v4(),
        user_id: user.id,
        bearer_token: token.clone(),
        refresh_token,
        token_expiration: Utc::now() + Duration::hours(1),
        refresh_token_expiration: Utc::now() + Duration::days(7),
        created_at: Utc::now(),
        last_accessed_at: Utc::now(),
        is_admin: user.is_admin,
        is_active: true,
        integrity_hash: String::new(), // Temporary placeholder
    };

    // Generate the integrity hash
    let integrity_hash = new_session.generate_integrity_hash();

    let new_session = session::ActiveModel {
        id: Set(new_session.id),
        user_id: Set(new_session.user_id),
        bearer_token: Set(new_session.bearer_token),
        refresh_token: Set(new_session.refresh_token),
        token_expiration: Set(new_session.token_expiration),
        refresh_token_expiration: Set(new_session.refresh_token_expiration),
        created_at: Set(new_session.created_at),
        last_accessed_at: Set(new_session.last_accessed_at),
        is_admin: Set(new_session.is_admin),
        is_active: Set(new_session.is_active),
        integrity_hash: Set(integrity_hash),
    };

    match new_session.insert(&db).await {
        Ok(_) => {
            tracing::info!("Session created from session handler successfully for user: {}", user.id);
            Ok(SessionResponse { token })
        }
        Err(e) => {
            tracing::error!("Error inserting new session: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn validate_session(
    Extension(db): Extension<DatabaseConnection>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<impl IntoResponse, StatusCode> {
    let token = bearer.token().to_string();
    
    let session = match session::Entity::find()
        .filter(session::Column::BearerToken.eq(token))
        .one(&db)
        .await
    {
        Ok(Some(session)) => session,
        Ok(None) => {
            tracing::warn!("No session found for token");
            return Err(StatusCode::UNAUTHORIZED);
        },
        Err(e) => {
            tracing::error!("Database error when fetching session: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if !session.is_active || !session.verify_integrity() {
        tracing::warn!("Session is inactive or failed integrity check");
        return Err(StatusCode::UNAUTHORIZED);
    }

    if session.token_expiration < Utc::now() {
        tracing::warn!("Session has expired");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Update last_accessed_at
    let mut updated_session: session::ActiveModel = session.into();
    updated_session.last_accessed_at = Set(Utc::now());
    updated_session.update(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn delete_session(
    Extension(db): Extension<DatabaseConnection>,
    Extension(session): Extension<session::Model>,
) -> Result<impl IntoResponse, StatusCode> {
    session::Entity::delete_by_id(session.id)
        .exec(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn cleanup_expired_sessions(db: &DatabaseConnection) {
    let result = session::Entity::delete_many()
        .filter(session::Column::RefreshTokenExpiration.lt(Utc::now()))
        .exec(db)
        .await;

    match result {
        Ok(del) => tracing::info!("Cleaned up {} expired sessions", del.rows_affected),
        Err(e) => tracing::error!("Error cleaning up expired sessions: {:?}", e),
    }
}

pub async fn refresh_token(
    Extension(db): Extension<DatabaseConnection>,
    Extension(current_session): Extension<session::Model>,
) -> Result<Json<SessionResponse>, StatusCode> {
    tracing::info!("Refreshing token for user: {}", current_session.user_id);
    tracing::debug!("Current session: {:?}", current_session);

    if current_session.refresh_token_expiration < Utc::now() {
        tracing::warn!("Refresh token has expired for user: {}", current_session.user_id);
        return Err(StatusCode::UNAUTHORIZED);
    }

    let user = match user::Entity::find_by_id(current_session.user_id)
        .one(&db)
        .await {
            Ok(Some(user)) => {
                tracing::debug!("User found: {:?}", user);
                user
            },
            Ok(None) => {
                tracing::error!("User not found for session: {:?}", current_session);
                return Err(StatusCode::UNAUTHORIZED);
            },
            Err(e) => {
                tracing::error!("Database error when finding user: {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

    let new_token = match generate_jwt(&user) {
        Ok(token) => {
            tracing::debug!("New token generated: {}", token);
            token
        },
        Err(e) => {
            tracing::error!("Error generating JWT: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let new_expiration = Utc::now() + chrono::Duration::hours(1);

    let mut session: session::ActiveModel = current_session.into();
    session.bearer_token = Set(new_token.clone());
    session.token_expiration = Set(new_expiration);
    session.last_accessed_at = Set(Utc::now());

    match session.update(&db).await {
        Ok(updated_session) => {
            tracing::info!("Session updated successfully: {:?}", updated_session);
            Ok(Json(SessionResponse { token: new_token }))
        },
        Err(e) => {
            tracing::error!("Error updating session: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}