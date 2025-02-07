use sea_orm::{DatabaseConnection, Set, ActiveModelTrait};
use uuid::Uuid;
use chrono::Utc;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::json;
use tower::ServiceExt;
use hyper::body::Bytes;


use crate::entities::{directory_type, directory};

pub async fn create_test_directory_type(db: &DatabaseConnection) -> directory_type::Model {
    let directory_type_id = Uuid::new_v4();
    let new_directory_type = directory_type::ActiveModel {
        id: Set(directory_type_id),
        name: Set("Test Directory Type".to_string()),
        description: Set("Test Directory Type Description".to_string()),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    new_directory_type.insert(db)
        .await
        .expect("Failed to create test directory type")
}

pub async fn create_test_directory(
    db: &DatabaseConnection, 
    directory_type_id: Uuid
) -> directory::Model {
    let directory_id = Uuid::new_v4();
    let new_directory = directory::ActiveModel {
        id: Set(directory_id),
        directory_type_id: Set(directory_type_id),
        name: Set("Test Directory".to_string()),
        domain: Set("test.example.com".to_string()),
        description: Set("Test Directory Description".to_string()),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    new_directory.insert(db)
        .await
        .expect("Failed to create test directory")
    
}

pub async fn register_test_user(
    app: &Router,
    directory_id: Uuid,
    username: &str,
) -> (StatusCode, String) {
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/register")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "directory_id": directory_id,
                        "username": username,
                        "first_name": "Test",
                        "last_name": "User",
                        "email": format!("{}@example.com", username),
                        "password": "password123",
                        "phone": "1234567890"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body = match response.into_body().data().await {
        Some(Ok(data)) => String::from_utf8_lossy(&data).to_string(),
        Some(Err(e)) => {
            panic!("Failed to collect response body: {}", e);
        }
        None => {
            panic!("Failed to collect response body");
        }
    };

    (status, body)
}
