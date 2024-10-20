use axum::{
    body::Body,
    http::{Method, Request, StatusCode, Uri},
    response::Response,
    middleware::Next,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use crate::handlers::request_logs::log_request;
use crate::models::request_log::{RequestType, RequestStatus};
use uuid::Uuid;

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
        req: &Request<B>,
    ) -> Result<(), StatusCode> {
        let method = req.method().clone();
        tracing::debug!("method in log request function: {:?}", method);
        tracing::debug!("uri in log request function: {:?}", req.uri().path());

        if req.uri().path() == "/validate-session" {
            tracing::debug!("nerrrr");
            return Ok(());
        } else {
            tracing::debug!("yerrrr");
        }

        let request_id = Uuid::new_v4();
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

        tracing::info!("Request received: ID: {}, Method: {}, URI: {}, User ID: {:?}, IP: {}, Type: {:?}",
            request_id, method, uri, user_id, ip_address, request_type);
        if let Err(e) = log_request(method, uri, StatusCode::OK, user_id, &user_agent, &ip_address, request_type, RequestStatus::Success, None, &self.db).await {
            eprintln!("Failed to log request: {}", e);
        }

        // We'll log the request status and failure reason in a separate function call after the response is generated

        Ok(())
    }

    pub async fn log_response(
        &self,
        response: &Response,
        method: Method,
        uri: Uri,
        user_id: Option<Uuid>,
        user_agent: &str,
        ip_address: &str,
        request_type: RequestType,
    ) -> Result<(), StatusCode> {
        let status = response.status();
        let request_status = if status.is_success() {
            RequestStatus::Success
        } else {
            RequestStatus::Failure
        };
        let failure_reason = if status.is_client_error() || status.is_server_error() {
            Some(status.canonical_reason().unwrap_or("Unknown error").to_string())
        } else {
            None
        };

        if let Err(e) = log_request(
            method,
            uri,
            status,
            user_id,
            user_agent,
            ip_address,
            request_type,
            request_status,
            failure_reason,
            &self.db
        ).await {
            tracing::error!("Failed to log request: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }

        Ok(())
    }
}
