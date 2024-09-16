use axum::{
    middleware::Next,
    response::Response,
    http::StatusCode,
    extract::Extension,
};
use crate::entities::user;

pub async fn admin_middleware<B>(
    req: axum::http::Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Get the user from the request extensions
    let user = req.extensions().get::<user::Model>().ok_or(StatusCode::UNAUTHORIZED)?;

    // Check if the user is an admin
    if !user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    // If the user is an admin, proceed to the next handler
    Ok(next.run(req).await)
}
