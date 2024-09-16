use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use sea_orm::Database;
use tower::ServiceExt;
use serde_json::json;

// Import the necessary modules from your crate
use crate::{api, db, migration};

async fn setup_test_app() -> Router {
    // Create a test database
    let database_url = "sqlite::memory:";
    let db = Database::connect(database_url).await.unwrap();

    // Run migrations
    migration::Migration::up(&db, None).await.unwrap();

    // Create the app
    api::router(db)
}

#[tokio::test]
async fn test_register_user() {
    let app = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users/register")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "username": "testuser",
                        "email": "test@example.com",
                        "password": "password123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

// Add more test functions for other API endpoints