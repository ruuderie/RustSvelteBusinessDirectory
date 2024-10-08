use axum::{
    middleware::Next,
    response::Response,
    http::{StatusCode, Request, Method},
    Extension,
};
use crate::entities::{user, session, user_account, profile, directory};
use sea_orm::{EntityTrait, DatabaseConnection, QueryFilter, ColumnTrait, Set};
use uuid::Uuid;
use chrono::Utc;
use axum::http;
use axum::extract::State;
use crate::handlers::request_logs::log_request;
use crate::models::request_log::RequestType;
use http::header;
use crate::models::request_log::RequestStatus;  // Import RequestStatus
use crate::middleware::rate_limiter::RateLimiter;

pub async fn auth_middleware<B>(
    State(db): State<DatabaseConnection>,
    Extension(rate_limiter): Extension<RateLimiter>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let path = req.uri().path().to_owned();
    let method = req.method().clone();
    let uri = req.uri().clone();

    tracing::info!("Processing request: {} {}", method, path);

    let (user_id, user_agent, ip_address) = {
        let headers = req.headers();
        let user_agent = headers
            .get(header::USER_AGENT)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("Unknown")
            .to_string();
        let ip_address = headers
            .get("x-forwarded-for")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("Unknown")
            .to_string();
        let user_id = req.extensions().get::<user::Model>().map(|user| user.id);
        (user_id, user_agent, ip_address)
    };

    let request_type = if path == "/login" { RequestType::Login } else { RequestType::API };

    if is_public_route(&path) {
        tracing::debug!("Public route detected, applying rate limiting");
        match rate_limiter.check_rate_limit(req, next).await {
            Ok(response) => {
                log_request(
                    method,
                    uri,
                    response.status(),
                    None,
                    &user_agent,
                    &ip_address,
                    request_type,
                    RequestStatus::Success,
                    None,
                    &db
                ).await?;
                return Ok(response);
            },
            Err(status) => {
                log_request(
                    method,
                    uri,
                    status,
                    None,
                    &user_agent,
                    &ip_address,
                    request_type,
                    RequestStatus::Failure,
                    Some("Rate limit exceeded".to_string()),
                    &db
                ).await?;
                return Err(status);
            }
        }
    }

    tracing::debug!("Authenticating request");
    let token = extract_token(&req);
    tracing::debug!("Token extracted: {}", token.is_some());

    let session = match validate_session(&db, token).await {
        Ok(session) => {
            tracing::debug!("Session validated successfully");
            session
        },
        Err(status) => {
            tracing::warn!("Session validation failed: {:?}", status);
            return Err(status);
        }
    };

    let user = match get_user(&db, &session).await {
        Ok(user) => {
            tracing::debug!("User retrieved successfully: {:?}", user.id);
            user
        },
        Err(status) => {
            tracing::warn!("Failed to retrieve user: {:?}", status);
            return Err(status);
        }
    };

    if req.uri().path().starts_with("/api/admin") && !user.is_admin {
        tracing::warn!("Non-admin user {:?} attempted to access admin route", user.id);
        return Err(StatusCode::FORBIDDEN);
    }

    if let Err(e) = update_session(&db, &session).await {
        tracing::error!("Failed to update session: {:?}", e);
        return Err(e);
    }

    tracing::debug!("Inserting user and session into request extensions");
    req.extensions_mut().insert(user.clone());
    req.extensions_mut().insert(session.clone());

    let directory_ids = match get_user_directory_ids(&db, &user).await {
        Ok(ids) => {
            tracing::debug!("Retrieved {} directory IDs for user", ids.len());
            ids
        },
        Err(e) => {
            tracing::error!("Failed to get user directory IDs: {:?}", e);
            return Err(e);
        }
    };
    req.extensions_mut().insert(directory_ids);

    tracing::debug!("Executing next middleware");
    let response = next.run(req).await;
    let status_code = response.status();

    tracing::info!("Request completed: {} {} - Status: {}", method, path, status_code);

    if let Err(e) = log_request(
        method,
        uri,
        status_code,
        user_id,
        &user_agent,
        &ip_address,
        request_type,
        RequestStatus::Success,
        None,
        &db
    ).await {
        tracing::error!("Failed to log request: {:?}", e);
    }

    Ok(response)
}

fn is_public_route(path: &str) -> bool {
    let public_routes = vec!["/login", "/register", "/refresh-token", "/api/listings", "/api/listing/"];
    public_routes.iter().any(|route| path.starts_with(route))
}

fn extract_token<B>(req: &Request<B>) -> Option<String> {
    req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        })
}

async fn validate_session(db: &DatabaseConnection, token: Option<String>) -> Result<session::Model, StatusCode> {
    let session = session::Entity::find()
        .filter(session::Column::BearerToken.eq(token.unwrap_or_default()))
        .one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !session.is_active || !session.verify_integrity() || session.token_expiration < Utc::now() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(session)
}

async fn get_user(db: &DatabaseConnection, session: &session::Model) -> Result<user::Model, StatusCode> {
    user::Entity::find_by_id(session.user_id)
        .one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)
}

async fn update_session(db: &DatabaseConnection, session: &session::Model) -> Result<(), StatusCode> {
    session::Entity::update(session::ActiveModel {
        id: Set(session.id),
        last_accessed_at: Set(Utc::now()),
        ..Default::default()
    })
    .exec(db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

async fn get_user_directory_ids(db: &DatabaseConnection, user: &user::Model) -> Result<Vec<Uuid>, StatusCode> {
    let user_accounts: Vec<Uuid> = user_account::Entity::find()
        .filter(user_account::Column::UserId.eq(user.id))
        .all(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|user_account| user_account.account_id)
        .collect();
    // get profile from account id on user_account
    let profiles = profile::Entity::find()
        .filter(profile::Column::AccountId.is_in(user_accounts))
        .all(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // get directory from profile
    let directories = directory::Entity::find()
        .filter(directory::Column::Id.is_in(profiles.into_iter().map(|profile| profile.directory_id)))
        .all(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    return Ok(directories.into_iter().map(|directory| directory.id).collect());
}