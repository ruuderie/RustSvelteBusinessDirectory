use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
    middleware::Next,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use crate::handlers::request_logs::log_request;
use crate::models::request_log::RequestType;

#[derive(Clone)]
pub struct RequestLogger {
    db: Arc<DatabaseConnection>,
}

impl RequestLogger {
    pub fn new(db: DatabaseConnection) -> Self {
        RequestLogger {
            db: Arc::new(db),
        }
    }

    pub async fn log_request<B>(
        &self,
        req: Request<B>,
        next: Next<B>,
    ) -> Result<Response, StatusCode> {
        let method = req.method().clone();
        let uri = req.uri().clone();
        let headers = req.headers().clone();
        let user_id = req.extensions().get::<crate::entities::user::Model>().map(|user| user.id);
        let ip_address = headers
            .get("x-forwarded-for")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("Unknown")
            .to_string();
        let user_agent = headers
            .get("user-agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("Unknown")
            .to_string();

        let request_type = if uri.path() == "/login" {
            RequestType::Login
        } else {
            RequestType::API
        };

        let response = next.run(req).await;

        // Log the request after it has been processed
        if let Err(e) = log_request(
            method,
            uri,
            response.status(),
            user_id,
            &user_agent,
            &ip_address,
            request_type,
            &self.db
        ).await {
            tracing::error!("Failed to log request: {:?}", e);
        }

        Ok(response)
    }
}