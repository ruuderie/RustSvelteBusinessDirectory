use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use sea_orm::{DatabaseConnection, EntityTrait, Condition, ColumnTrait, QueryFilter, Set, ActiveValue};
use crate::entities::business::{self, Entity as Business};
use crate::entities::user::{self, Entity as User};
use crate::models::{BusinessSearch, UserRegistration, UserLogin};
use crate::auth::{hash_password, verify_password, generate_jwt};
use chrono::NaiveDateTime;
use chrono::Utc;

pub fn router() -> Router<DatabaseConnection> {
    Router::new()
        .route("/businesses", get(get_businesses))
        .route("/businesses/search", get(search_businesses))
        .route("/businesses/:id", get(get_business_by_id))
        .route("/users/register", post(register_user))
        .route("/users/login", post(login_user))
}

async fn get_businesses(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<business::Model>>, axum::http::StatusCode> {
    println!("Fetching all businesses");
    match Business::find().all(&db).await {
        Ok(businesses) => {
            println!("Businesses: {:?}", businesses.len());
            Ok(Json(businesses))
        }
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn search_businesses(
    State(db): State<DatabaseConnection>,
    Query(params): Query<BusinessSearch>,
) -> Result<Json<Vec<business::Model>>, axum::http::StatusCode> {
    let businesses = Business::find()
        .filter(
            Condition::any()
                .add(business::Column::Name.contains(&params.q))
                .add(business::Column::Category.contains(&params.q)),
        )
        .all(&db)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(businesses))
}

async fn get_business_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<business::Model>, axum::http::StatusCode> {
    match Business::find_by_id(id).one(&db).await {
        Ok(Some(business)) => Ok(Json(business)),
        Ok(None) => Err(axum::http::StatusCode::NOT_FOUND),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn register_user(
    State(db): State<DatabaseConnection>,
    Json(user_data): Json<UserRegistration>,
) -> Result<Json<user::Model>, axum::http::StatusCode> {
    println!("Received registration request for email: {}", user_data.email);

    // Check if user already exists
    let existing_user = match User::find()
        .filter(user::Column::Email.eq(&user_data.email))
        .one(&db)
        .await
    {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Database error when checking for existing user: {:?}", e);
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if existing_user.is_some() {
        println!("User with email {} already exists", user_data.email);
        return Err(axum::http::StatusCode::CONFLICT);
    }

    println!("Hashing password for new user");
    let hashed_password = match hash_password(&user_data.password) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Error hashing password: {:?}", e);
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    println!("Creating new user active model");
    let new_user = user::ActiveModel {
        username: Set(user_data.username),
        email: Set(user_data.email),
        password_hash: Set(hashed_password),
        created_at: Set(Utc::now()),
        ..Default::default()
    };

    println!("Inserting new user into database");
    match User::insert(new_user).exec(&db).await {
        Ok(result) => {
            println!("User inserted, fetching user data");
            match User::find_by_id(result.last_insert_id).one(&db).await {
                Ok(Some(user)) => {
                    println!("User registration successful");
                    Ok(Json(user))
                },
                Ok(None) => {
                    eprintln!("User not found after insertion");
                    Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                },
                Err(e) => {
                    eprintln!("Error fetching newly inserted user: {:?}", e);
                    Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        },
        Err(e) => {
            eprintln!("Error inserting new user: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn login_user(
    State(db): State<DatabaseConnection>,
    Json(login_data): Json<UserLogin>,
) -> Result<Json<String>, axum::http::StatusCode> {
    println!("Received login request for email: {}", login_data.email);
    let user = User::find()
        .filter(user::Column::Email.eq(&login_data.email))
        .one(&db)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    if verify_password(&login_data.password, &user.password_hash)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
    {
        println!("User authenticated");
        let token = generate_jwt(&user)
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(token))
    } else {
        println!("User authentication failed");
        Err(axum::http::StatusCode::UNAUTHORIZED)
    }
}
