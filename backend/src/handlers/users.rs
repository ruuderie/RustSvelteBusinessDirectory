use crate::entities::{
    ad_purchase::{self, Entity as AdPurchase},
    profile::{self, Entity as Profile},
    user::{self, Entity as User},
    user_account::{self, Entity as UserAccount},
    session::{self, Entity as Session},
    account::{self, Entity as Account},
};
use axum::{
    body::Body, extract::{Extension, Json, Path, State, TypedHeader}, headers::{HeaderMap, UserAgent}, http::{header::USER_AGENT, StatusCode}, response::IntoResponse, routing::{get, post}, Router
};
use serde::{Deserialize, Serialize};
use crate::auth::{hash_password, verify_password, generate_jwt, generate_jwt_admin};
use crate::models::user::{UserLogin, UserRegistration};
use crate::handlers::sessions::{create_session, refresh_token, validate_session};
use crate::handlers::profiles::get_profile_by_id;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter, ActiveModelTrait};
use uuid::Uuid;
use chrono::{Utc, Duration};
use crate::handlers::request_logs::log_request;

#[derive(Deserialize)]
pub struct LoginCredentials {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub fn auth_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/login", post(login_user))
        .route("/register", post(register_user))
        .route("/logout", post(logout_user))
        .route("/refresh-token", post(refresh_token))
        .route("/validate-session", get(validate_session))
}

pub fn authenticated_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/profile", get(get_user_profile))
}

pub async fn get_user_profile(
    State(db): State<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path(id): Path<Uuid>,
) -> Result<Json<profile::Model>, StatusCode> {
    get_profile_by_id(Extension(db), Extension(current_user), Path(id)).await
}

pub async fn register_user(
    State(db): State<DatabaseConnection>,
    Json(user_data): Json<UserRegistration>,
) -> Result<Json<user::Model>, StatusCode> {
    tracing::info!("Received registration request for email: {}", user_data.email);

    let directory_id = user_data.directory_id;

    // Step 1: Check if a user already exists with the same email in the directory
    let existing_user = User::find()
        .filter(user::Column::Email.eq(&user_data.email))
        .one(&db)
        .await
        .map_err(|err| {
            tracing::error!("Database error when checking for existing user: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if existing_user.is_some() {
        tracing::warn!("User with email {} already exists in the system", user_data.email);
        return Err(StatusCode::CONFLICT);
    }

    // Step 2: Hash password and create a new user
    let hashed_password = hash_password(&user_data.password)
        .map_err(|err| {
            tracing::error!("Error hashing password: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Clone username and email before moving them
    let username = user_data.username.clone();
    let email = user_data.email.clone();

    // Step 3: Create the user
    let new_user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(username.clone()),
        first_name: Set(user_data.first_name),
        last_name: Set(user_data.last_name),
        phone: Set(user_data.phone),
        email: Set(email.clone()),
        password_hash: Set(hashed_password),
        is_admin: Set(false),
        is_active: Set(true),
        last_login: Set(None),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let inserted_user = new_user.insert(&db).await.map_err(|err| {
        tracing::error!("Database error when inserting user: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Step 4: Find or create the Profile for the directory
    let profile: Option<profile::Model> = profile::Entity::find()
        .filter(profile::Column::DirectoryId.eq(directory_id))
        .one(&db)
        .await
        .map_err(|err| {
            tracing::error!("Database error when finding profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let account_id = profile.clone().unwrap().account_id;

    let profile_id = if let Some(profile) = profile {
        profile.id
    } else {
        // Create a new Profile if not found
        let new_profile = profile::ActiveModel {
            id: Set(Uuid::new_v4()),
            account_id: Set(account_id.clone()),
            additional_info: Set(None),
            is_active: Set(true),
            directory_id: Set(directory_id),
            profile_type: Set(profile::ProfileType::Business), // Assuming it's a business profile
            display_name: Set(username),
            contact_info: Set(email),
            business_name: Set(None),
            business_address: Set(None),
            business_phone: Set(None),
            business_website: Set(None),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };

        let inserted_profile = new_profile.insert(&db).await.map_err(|err| {
            tracing::error!("Database error when inserting profile: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        inserted_profile.id
    };

    // Step 5: Create the UserAccount to link user and profile
    let new_user_account = user_account::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(inserted_user.id),
        account_id: Set(account_id),
        role: Set(user_account::UserRole::Owner),
        created_at: Set(Utc::now()),
        is_active: Set(true),
        updated_at: Set(Utc::now()),
    };

    new_user_account.insert(&db).await.map_err(|err| {
        tracing::error!("Database error when creating user profile: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(inserted_user))
}

pub async fn login_user(
    State(db): State<DatabaseConnection>,
    Json(credentials): Json<LoginCredentials>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::info!("Attempting to log in user: {}", credentials.email);

    let user = match User::find()
        .filter(user::Column::Email.eq(&credentials.email))
        .one(&db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            tracing::warn!("User not found for email: {}", credentials.email);
            return Err(StatusCode::UNAUTHORIZED);
        },
        Err(e) => {
            tracing::error!("Database error when finding user: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match verify_password(&credentials.password, &user.password_hash) {
        Ok(true) => tracing::info!("Password verified successfully"),
        Ok(false) => {
            tracing::warn!("Invalid password for user: {}", user.id);
            return Err(StatusCode::UNAUTHORIZED);
        },
        Err(e) => {
            tracing::error!("Error verifying password: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Use the create_session function from sessions.rs
    match create_session(Extension(db.clone()), Json(UserLogin {
        email: credentials.email.clone(),
        password: credentials.password.clone(),
    })).await {
        Ok(session_response) => {
            tracing::info!("Session created from user handler successfully for user: {}", user.id);
            Ok(Json(session_response))
        },
        Err(e) => {
            tracing::error!("Error creating session: {:?}", e);
            Err(e)
        }
    }
}

pub async fn logout_user(
    State(db): State<DatabaseConnection>,
    Extension(session): Extension<crate::entities::session::Model>,
) -> Result<impl IntoResponse, StatusCode> {
    if !session.verify_integrity() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    session::Entity::update(session::ActiveModel {
        id: Set(session.id),
        is_active: Set(false),
        ..Default::default()
    })
    .exec(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
