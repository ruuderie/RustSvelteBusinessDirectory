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
use super::test_utils;

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

    // Create test directory type and directory using utilities
    let directory_type = test_utils::create_test_directory_type(&db).await;
    let directory = test_utils::create_test_directory(&db, directory_type.id).await;

    // Register test user
    let (status, body) = test_utils::register_test_user(&app, directory.id, "testuser").await;

    if status != StatusCode::CREATED {
        println!("Test failed with status: {}", status);
        println!("Error response: {}", body);
    }

    assert_eq!(status, StatusCode::CREATED);
}

// Example of a test that builds on user registration
#[tokio::test]
async fn test_user_login() {
    let (app, db) = setup_test_app().await;

    // Setup prerequisites using utilities
    let directory_type = test_utils::create_test_directory_type(&db).await;
    let directory = test_utils::create_test_directory(&db, directory_type.id).await;
    
    // Register user first
    let (register_status, _) = test_utils::register_test_user(&app, directory.id, "logintest").await;
    assert_eq!(register_status, StatusCode::CREATED);

    // Now test login
    // ... rest of login test
}