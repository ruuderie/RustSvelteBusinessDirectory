use crate::entities::{
    ad_purchase::{self, Entity as AdPurchase},
    profile::{self, Entity as Profile},
    user::{self, Entity as User},
    user_account::{self, Entity as UserAccount},
};
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};
use crate::auth::{hash_password, verify_password, generate_jwt, generate_jwt_admin};
use crate::models::user::{UserLogin, UserRegistration};
use crate::handlers::profiles::get_profile_by_id;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter, ActiveModelTrait};
use uuid::Uuid;
use chrono::{Utc, Duration};

#[derive(Deserialize)]
pub struct LoginCredentials {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(login_user))
        .route("/register", post(register_user))
}

pub fn authenticated_routes() -> Router {
    Router::new()
        // We can add any user-related routes that require authentication
        .route("/profile", get(get_user_profile))
}

pub async fn get_user_profile(
    Extension(db): Extension<DatabaseConnection>,
    Extension(current_user): Extension<user::Model>,
    Path(id): Path<Uuid>,
) -> Result<Json<profile::Model>, StatusCode> {
    get_profile_by_id(Extension(db), Extension(current_user), Path(id)).await
}

pub async fn register_user(
    Extension(db): Extension<DatabaseConnection>,
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
        email: Set(email.clone()),
        password_hash: Set(hashed_password),
        is_admin: Set(false),
        is_active: Set(true),
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
    Extension(db): Extension<DatabaseConnection>,
    Json(credentials): Json<LoginCredentials>,
) -> Result<Json<LoginResponse>, StatusCode> {
    tracing::info!("Login attempt for email: {}", credentials.email);
    
    let user = match user::Entity::find()
        .filter(user::Column::Email.eq(credentials.email.clone()))
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
    tracing::info!("User found with id: {}", user.id);

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

    //if the user is not an admin, proceed as normal
    if !user.is_admin {
        tracing::info!("User is not an admin, proceeding as normal");
        //get the user's account id
        let user_account = user_account::Entity::find()
            .filter(user_account::Column::UserId.eq(user.id))
            .one(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

            
        // Fetch the user's profile
        let profile = profile::Entity::find()
            .filter(profile::Column::AccountId.eq(user_account.account_id))
            .one(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        let token = generate_jwt(&user, &profile)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(LoginResponse { token }))
    } else {
        //NOTE: admins don't need a profile, so we can skip the profile part but in the future 
        // we will need to add a profile to the NON-ROOT admin users
        // users who administer one or more directories and not the entire application
        
        tracing::info!("User is an admin, generating admin token");

        //if the user is an admin, generate a different token
        let token = generate_jwt_admin(&user)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(LoginResponse { token }))
    }
}