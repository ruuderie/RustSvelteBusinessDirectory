use std::sync::Arc;
use std::time::{Duration, Instant};
use axum::http::{Request, StatusCode};
use axum::response::Response;
use dashmap::DashMap;
use axum::extract::FromRequest;
use axum::body::{Body, boxed};

const MAX_REQUESTS: u32 = 100;
const WINDOW_SIZE: Duration = Duration::from_secs(60);

#[derive(Clone)]
pub struct RateLimiter {
    store: Arc<DashMap<String, (Instant, u32)>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        tracing::info!("Initializing rate limiter");
        RateLimiter {
            store: Arc::new(DashMap::new()),
        }
    }

    pub async fn check_rate_limit<B>(&self, req: &Request<B>) -> Result<(), StatusCode> 
    where
        B: axum::body::HttpBody + Send + 'static,
    {
        let ip = req
            .headers()
            .get("x-forwarded-for")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        let now = Instant::now();
        let mut should_allow = true;

        self.store.entry(ip.clone()).and_modify(|(window_start, count)| {
            if now.duration_since(*window_start) > WINDOW_SIZE {
                *window_start = now;
                *count = 1;
            } else {
                *count += 1;
                if *count > MAX_REQUESTS {
                    should_allow = false;
                }
            }
        }).or_insert((now, 1));

        tracing::debug!("Request allowed: {}", should_allow);
        if !should_allow {
            tracing::debug!("Rate limit exceeded for IP: {}", ip);
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }

        Ok(())
    }
}
