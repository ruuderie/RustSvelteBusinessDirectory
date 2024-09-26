use axum::{
    middleware::Next,
    response::Response,
    http::{StatusCode, Request},
    Extension,
};
use crate::auth::{validate_jwt, Claims, ClaimsAdmin};
use crate::entities::{user, user_account, profile, account};
use sea_orm::{EntityTrait, DatabaseConnection, QueryFilter, ColumnTrait};
use uuid::Uuid;
use crate::models::user::User;

#[derive(Debug)]
enum ValidClaims {
    User(Claims),
    Admin(ClaimsAdmin),
}

pub async fn auth_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    tracing::debug!("Auth middleware called for path: {}", req.uri().path());

    let path = req.uri().path();

    // Special handling for login and register routes
    if path == "/login" || path == "/register" {
        tracing::debug!("Login/Register route detected, proceeding without token validation");
        return Ok(next.run(req).await);
    }

    // Check if the route is public
    if is_public_route(path) {
        tracing::debug!("Public route detected, skipping authentication");
        return Ok(next.run(req).await);
    } else {
        tracing::debug!("Private route detected, checking authentication");
    }

    // Extract the Authorization header
    let auth_header = req.headers().get(axum::http::header::AUTHORIZATION)
        .ok_or_else(|| {
            tracing::error!("No Authorization header found");
            StatusCode::UNAUTHORIZED
        })?;

    // Parse the token
    let token = auth_header.to_str()
        .map_err(|e| {
            tracing::error!("Failed to parse Authorization header: {:?}", e);
            StatusCode::UNAUTHORIZED
        })?
        .trim_start_matches("Bearer ")
        .to_string();
    tracing::debug!("Token: {}", token);

    // Validate the token and extract claims
    let claims = validate_jwt::<Claims>(&token)
        .map(ValidClaims::User)
        .or_else(|_| validate_jwt::<ClaimsAdmin>(&token).map(ValidClaims::Admin))
        .map_err(|e| {
            tracing::error!("Failed to validate JWT: {:?}", e);
            StatusCode::UNAUTHORIZED
        })?;

    let user_id = match &claims {
        ValidClaims::User(user_claims) => &user_claims.sub,
        ValidClaims::Admin(admin_claims) => &admin_claims.sub,
    };

    tracing::debug!("JWT validated successfully for user: {}", user_id);

    // Fetch the user from the database
    let user_id = Uuid::parse_str(user_id).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let db = req.extensions().get::<DatabaseConnection>().unwrap().clone();
    let user = user::Entity::find()
        .filter(user::Column::Id.eq(user_id))
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;
    tracing::debug!("User {:?} found", user.id);
    // Insert the user into the request extensions
    req.extensions_mut().insert(user.clone());

    // Check if the user is an admin
    if !is_admin(&user, &db).await {
        tracing::error!("User is not an admin");
        return Err(StatusCode::FORBIDDEN);
    }

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

    // Attach user and directory_ids to request extensions
    req.extensions_mut().insert(directory_ids);

    // Proceed to the next handler
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
    Extension(user): Extension<user::Model>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    println!("Admin check middleware called for path: {}", request.uri().path());
    tracing::debug!("Admin check middleware called for path: {}", request.uri().path());
    tracing::debug!("User {:?} trying to access admin route", user.id);
    
    if request.uri().path().starts_with("/api/admin") && !user.is_admin {
        tracing::error!("User is not an admin");
        Err(StatusCode::FORBIDDEN)
    } else {
        tracing::debug!("User is an admin");
        Ok(next.run(request).await)
    }
}