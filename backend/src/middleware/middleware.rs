use axum::{
    middleware::Next,
    response::Response,
    http::{StatusCode, Request},
    Extension,
};
use crate::auth::{validate_jwt, Claims};
use crate::entities::{user, user_account, profile, account, session};
use sea_orm::{EntityTrait, DatabaseConnection, QueryFilter, ColumnTrait, Set, ActiveModelTrait};
use uuid::Uuid;
use crate::models::user::User;
use chrono::Utc;
use axum::http;

pub async fn auth_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    tracing::debug!("Auth middleware called for path: {}", req.uri().path());

    let path = req.uri().path();

    if path == "/login" || path == "/register" || path == "/refresh-token" || is_public_route(path) {
        tracing::debug!("Public route detected, skipping authentication");
        return Ok(next.run(req).await);
    }

    let token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });
    tracing::info!("Received token: {:?}", token);

    let db = req.extensions().get::<DatabaseConnection>().unwrap().clone();
    let session = match session::Entity::find()
        .filter(session::Column::BearerToken.eq(token.clone().unwrap_or_default()))
        .one(&db)
        .await
    {
        Ok(Some(session)) => {
            if session.token_expiration < Utc::now() {
                tracing::info!("Session details: {:?}", session);
                tracing::info!("Session is active: {:?}", session.is_active);
                tracing::info!("Session integrity: {:?}", session.verify_integrity());
                tracing::info!("Session token expiration: {:?}", session.token_expiration);
                tracing::info!("Session refresh token expiration: {:?}", session.refresh_token_expiration);
                tracing::info!("time now: {:?}", Utc::now());
                tracing::warn!("Token has expired. Please refresh the token.");
                return Err(StatusCode::UNAUTHORIZED);
            }
            session
        },
        Ok(None) => {
            tracing::error!("No session found for token: {:?}", token);
            return Err(StatusCode::UNAUTHORIZED);
        },
        Err(e) => {
            tracing::error!("Database error when fetching session: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    tracing::debug!("Session found: {:?}", session.id);
    tracing::debug!("Session is active: {:?}", session.is_active);
    tracing::debug!("Session integrity: {:?}", session.verify_integrity());
    tracing::debug!("Session token expiration: {:?}", session.token_expiration);
    tracing::debug!("Session refresh token expiration: {:?}", session.refresh_token_expiration);


    if !session.is_active || !session.verify_integrity() {
        
        tracing::error!("Session is inactive or failed integrity check");
        return Err(StatusCode::UNAUTHORIZED);
    }

    if session.token_expiration < Utc::now() {
        if session.refresh_token_expiration > Utc::now() {
            // Soft expiration: Refresh the session
            let user = user::Entity::find_by_id(session.user_id)
                .one(&db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::UNAUTHORIZED)?;

            let new_token = crate::auth::generate_jwt(&user)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let new_expiration = Utc::now() + chrono::Duration::hours(1);
            
            let updated_session = session::ActiveModel {
                id: Set(session.id),
                bearer_token: Set(new_token.clone()),
                token_expiration: Set(new_expiration),
                last_accessed_at: Set(Utc::now()),
                ..Default::default()
            };
            
            let updated_session: session::Model = updated_session.update(&db).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            let integrity_hash = updated_session.generate_integrity_hash();
            session::Entity::update(session::ActiveModel {
                id: Set(updated_session.id),
                integrity_hash: Set(integrity_hash),
                ..Default::default()
            })
            .exec(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Update the Authorization header with the new token
            req.headers_mut().insert(
                http::header::AUTHORIZATION,
                http::HeaderValue::from_str(&format!("Bearer {}", new_token)).unwrap(),
            );
        } else {
            // Hard expiration: Force re-authentication
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        tracing::debug!("Session is active and not expired");

        // Update last_accessed_at
        session::Entity::update(session::ActiveModel {
            id: Set(session.id),
            last_accessed_at: Set(Utc::now()),
            ..Default::default()
        })
        .exec(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    let user = user::Entity::find_by_id(session.user_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    tracing::debug!("User {:?} found", user.id);

    req.extensions_mut().insert(user.clone());
    req.extensions_mut().insert(session.clone());

    // Fetch the user's profiles and their associated directories
    let user_accounts = user_account::Entity::find()
        .filter(user_account::Column::UserId.eq(user.id))
        .find_with_related(account::Entity)
        .all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let directory_ids: Vec<Uuid> = user_accounts
        .iter()
        .filter_map(|(_, accounts)| accounts.first())
        .filter_map(|account| Some(account.directory_id))
        .collect();

    // Attach directory_ids to request extensions
    req.extensions_mut().insert(directory_ids);

    Ok(next.run(req).await)
}

async fn is_admin(user: &user::Model, db: &DatabaseConnection) -> bool {
    user.is_admin
}

fn is_public_route(path: &str) -> bool {
    tracing::debug!("Checking if path is public: {}", path);
    let public_routes = vec![
        "/api/directories",
        "/api/listings",
        "/api/listing/",
    ];

    let is_public = public_routes.iter().any(|route| path.starts_with(route));
    tracing::debug!("Path is public: {}", is_public);
    is_public
}

pub async fn admin_check_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    tracing::debug!("Admin check middleware called for path: {}", req.uri().path());
    tracing::debug!("Request headers: {:?}", req.headers());
    tracing::debug!("Request extensions: {:?}", req.extensions());
    let token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });
    tracing::debug!("Extracted token: {:?}", token);

    let db = req.extensions().get::<DatabaseConnection>().unwrap().clone();
    let session = match session::Entity::find()
        .filter(session::Column::BearerToken.eq(token.clone().unwrap_or_default()))
        .one(&db)
        .await
    {
        Ok(Some(session)) => {
            tracing::debug!("Session found: {:?}", session.id);
            Some(session)
        },
        Ok(None) => {
            tracing::error!("No session found for token");
            // Debug: Print all sessions in the database
            let all_sessions = session::Entity::find().all(&db).await.unwrap_or_default();
            tracing::debug!("All sessions in database: {:?}", all_sessions);
            None
        },
        Err(e) => {
            tracing::error!("Database error when fetching session: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let user = match check_user_auth(&db, session.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            tracing::error!("User not found for session");
            return Err(StatusCode::UNAUTHORIZED);
        }
        Err(e) => {
            tracing::error!("Error checking user auth: {:?}", e);
            return Err(e);
        }
    };

    if req.uri().path().starts_with("/api/admin") && !user.is_admin {
        tracing::error!("User {:?} is not an admin", user.id);
        Err(StatusCode::FORBIDDEN)
    } else {
        tracing::info!("User {:?} is authorized", user.id);
        Ok(next.run(req).await)
    }
}

async fn check_user_auth(db: &DatabaseConnection, session: Option<session::Model>) -> Result<Option<user::Model>, StatusCode> {
    let user = match session {
        Some(session) => {
            user::Entity::find_by_id(session.user_id)
                .one(db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        },
        None => None,
    };

    Ok(user)
}