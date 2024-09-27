use axum::{
    middleware::Next,
    response::Response,
    http::StatusCode,
    extract::Extension,
};
use crate::entities::user;

pub async fn admin_middleware<B>(
    mut req: axum::http::Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    tracing::debug!("Admin middleware called for path: {}", req.uri().path());
    
    // Get the user from the request extensions
    let user = match req.extensions().get::<user::Model>().cloned() {
        Some(user) => user,
        None => {
            tracing::error!("admin_middleware:User not found in request extensions");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    
    tracing::debug!("User: {:?}", user.id);
    
    // Check if the user is an admin
    if !user.is_admin {
        tracing::warn!("User {:?} is not an admin", user.id);
        return Err(StatusCode::FORBIDDEN);
    }
    
    tracing::debug!("User {:?} is an admin, proceeding to the next handler", user.id);
    
    // If the user is an admin, proceed to the next handler
    Ok(next.run(req).await)
}
