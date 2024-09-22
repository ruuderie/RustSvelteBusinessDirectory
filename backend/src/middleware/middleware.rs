use axum::{
    middleware::Next,
    response::Response,
    http::{StatusCode, Request},
};
use crate::auth::validate_jwt;
use crate::entities::{user, user_account, profile, account};
use sea_orm::{EntityTrait, DatabaseConnection, QueryFilter, ColumnTrait};
use uuid::Uuid;

pub async fn auth_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    tracing::debug!("Auth middleware called for path: {}", req.uri().path());
    // Extract the Authorization header
    let auth_header = req.headers().get(axum::http::header::AUTHORIZATION)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Parse the token
    let token = auth_header.to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .trim_start_matches("Bearer ")
        .to_string();

    // Validate the token and extract claims
    let claims = validate_jwt(&token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Fetch the user from the database
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let db = req.extensions().get::<DatabaseConnection>().unwrap().clone();
    let user = user::Entity::find()
        .filter(user::Column::Id.eq(user_id))
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

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
    req.extensions_mut().insert(user);
    req.extensions_mut().insert(directory_ids);

    // Proceed to the next handler
    Ok(next.run(req).await)
}