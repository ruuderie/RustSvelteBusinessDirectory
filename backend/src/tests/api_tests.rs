use axum::{
    body::{Body, HttpBody},
    http::{Request, StatusCode},
    Router,
    extract::FromRequest,
};
use sea_orm::{Database, DatabaseConnection, EntityTrait, Set, ActiveModelTrait, ConnectionTrait};
use sea_orm_migration::MigratorTrait;
use tower::ServiceExt;
use serde_json::json;
use std::env;
use uuid::Uuid;
use crate::{api, migration};
use hyper::body::Bytes;
use chrono::Utc;

async fn setup_test_app() -> (Router, DatabaseConnection) {
    // Load test database URL from environment
    let database_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/business_directory_test".to_string());

    // Create a test database connection
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    // Drop all tables and indexes
    db.execute_unprepared(
        "DO $$ DECLARE r RECORD;
        BEGIN
            FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = current_schema()) LOOP
                EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
            END LOOP;
            FOR r IN (SELECT indexname FROM pg_indexes WHERE schemaname = current_schema()) LOOP
                EXECUTE 'DROP INDEX IF EXISTS ' || quote_ident(r.indexname) || ' CASCADE';
            END LOOP;
        END $$;"
    )
    .await
    .expect("Failed to drop tables and indexes");

    // Run migrations
    migration::Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    // Create the app
    (api::create_router(db.clone()), db)
}

async fn cleanup_database(db: &DatabaseConnection) {
    // Drop all tables and indexes
    db.execute_unprepared(
        "DO $$ DECLARE r RECORD;
        BEGIN
            FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = current_schema()) LOOP
                EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
            END LOOP;
            FOR r IN (SELECT indexname FROM pg_indexes WHERE schemaname = current_schema()) LOOP
                EXECUTE 'DROP INDEX IF EXISTS ' || quote_ident(r.indexname) || ' CASCADE';
            END LOOP;
        END $$;"
    )
    .await
    .expect("Failed to drop tables and indexes");

    // Run migrations
    migration::Migrator::up(db, None)
        .await
        .expect("Failed to run migrations");
}

#[tokio::test]
async fn test_register_user() {
    let (app, db) = setup_test_app().await;

    // Create a test directory type first
    let directory_type_id = Uuid::new_v4();
    let new_directory_type = crate::entities::directory_type::ActiveModel {
        id: Set(directory_type_id),
        name: Set("Test Directory Type".to_string()),
        description: Set("Test Directory Type Description".to_string()),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let directory_type = new_directory_type.insert(&db)
        .await
        .expect("Failed to create test directory type");

    // Create a test directory
    let directory_id = Uuid::new_v4();
    let new_directory = crate::entities::directory::ActiveModel {
        id: Set(directory_id),
        directory_type_id: Set(directory_type.id),
        name: Set("Test Directory".to_string()),
        domain: Set("test.example.com".to_string()),
        description: Set("Test Directory Description".to_string()),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let directory = new_directory.insert(&db)
        .await
        .expect("Failed to create test directory");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/register")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "directory_id": directory.id,
                        "username": "testuser",
                        "first_name": "Test",
                        "last_name": "User",
                        "email": "test@example.com",
                        "password": "password123",
                        "phone": "1234567890"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Capture headers before consuming the response body
    let headers = response.headers().clone();
    let status = response.status();

    // Read and print the response body for debugging
    let body = match response.into_body().data().await {
        Some(Ok(data)) => String::from_utf8_lossy(&data).to_string(),
        Some(Err(err)) => {
            println!("Error reading body: {}", err);
            String::new()
        }
        None => {
            println!("Body is empty or not available");
            String::new()
        }
    };

    println!("Response headers: {:?}", headers);
    println!("Response status: {}", status);
    println!("Response body: {}", body);

    if status != StatusCode::CREATED {
        println!("Test failed with status: {}", status);
        println!("Error response: {}", body);
    }

    assert_eq!(status, StatusCode::CREATED);
}