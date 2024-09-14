use crate::entities::{user, user_profile, profile};
use sea_orm::{EntityTrait, DatabaseConnection, QueryFilter, ColumnTrait};
use axum::{
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use crate::auth::{validate_jwt, Claims};
use uuid::Uuid;

pub async fn auth_middleware<B>(
    mut req: axum::http::Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
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
    let directory_id = Uuid::parse_str(&claims.directory_id).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let db = req.extensions().get::<DatabaseConnection>().unwrap().clone();
    let user = user::Entity::find()
        .filter(user::Column::Id.eq(user_id))
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Fetch the user's profiles and their associated directories
    let user_profiles = user_profile::Entity::find()
        .filter(user_profile::Column::UserId.eq(user.id))
        .find_with_related(profile::Entity)
        .all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let directory_ids: Vec<Uuid> = user_profiles
        .into_iter()
        .map(|(_, profile)| profile.directory_id)
        .collect();

    // Attach user and directory_ids to request extensions
    req.extensions_mut().insert(user);
    req.extensions_mut().insert(directory_ids);

    // Proceed to the next handler
    Ok(next.run(req).await)
}