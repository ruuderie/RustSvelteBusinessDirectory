use crate::entities::{
/*  ad_placement::{self, Entity as AdPlacement},*/
    ad_purchase::{self, Entity as AdPurchase},
    profile::{self, Entity as Profile},
    user::{self, Entity as User},
    user_profile::{self, Entity as UserProfile},
};
use axum::{
    extract::{Extension, Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use crate::models::{UserLogin, UserRegistration};
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;
use chrono::Utc;
use crate::auth::{hash_password, verify_password, generate_jwt};

pub async fn register_user(
    State(db): State<DatabaseConnection>,
    Json(user_data): Json<UserRegistration>,
) -> Result<Json<user::Model>, axum::http::StatusCode> {
    println!("Received registration request for email: {}", user_data.email);

    let directory_id = user_data.directory_id;

    // Step 1: Check if a user already exists with the same email in the directory
    let existing_user = User::find()
        .filter(user::Column::Email.eq(&user_data.email))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Database error when checking for existing user: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if existing_user.is_some() {
        println!("User with email {} already exists in the system", user_data.email);
        return Err(axum::http::StatusCode::CONFLICT);
    }

    // Step 2: Hash password and create a new user
    let hashed_password = hash_password(&user_data.password)
        .map_err(|err| {
            eprintln!("Error hashing password: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Step 3: Create the user
    let new_user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(user_data.username),
        email: Set(user_data.email),
        password_hash: Set(hashed_password),
        is_admin: Set(false),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let inserted_user = new_user.insert(&db).await.map_err(|err| {
        eprintln!("Database error when inserting user: {:?}", err);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Step 4: Find or create the Profile for the directory
    let profile = profile::Entity::find()
        .filter(profile::Column::DirectoryId.eq(directory_id))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Database error when finding profile: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let profile_id = if let Some(profile) = profile {
        profile.id
    } else {
        // Create a new Profile if not found
        let new_profile = profile::ActiveModel {
            id: Set(Uuid::new_v4()),
            directory_id: Set(directory_id),
            // Fill out other necessary fields for the profile entity here...
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };

        let inserted_profile = new_profile.insert(&db).await.map_err(|err| {
            eprintln!("Database error when inserting profile: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

        inserted_profile.id
    };

    // Step 5: Create the UserProfile to link user and profile
    let new_user_profile = user_profile::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(inserted_user.id),
        profile_id: Set(profile_id),
        role: Set(user_profile::UserProfileRole::Owner),
        created_at: Set(Utc::now()),
    };

    new_user_profile.insert(&db).await.map_err(|err| {
        eprintln!("Database error when creating user profile: {:?}", err);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(inserted_user))
}

pub async fn login_user(
    State(db): State<DatabaseConnection>,
    Json(login_data): Json<UserLogin>,
) -> Result<Json<String>, axum::http::StatusCode> {
    println!("Received login request for email: {}", login_data.email);

    let directory_id = login_data.directory_id;

    // Find the user by email
    let user = User::find()
        .filter(user::Column::Email.eq(&login_data.email))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    // Verify password
    if verify_password(&login_data.password, &user.password_hash)
        .map_err(|err| {
            eprintln!("Error verifying password: {:?}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?
    {
        println!("User authenticated");

        // Fetch user's profiles and associated directory_ids
        let user_profiles = user_profile::Entity::find()
            .filter(user_profile::Column::UserId.eq(user.id))
            .find_with_related(profile::Entity)
            .all(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let directory_ids: Vec<Uuid> = user_profiles
            .into_iter()
            .map(|(_, profile)| profile.directory_id)
            .collect();

        // Generate JWT including user_id and directory_ids
        let token = generate_jwt(&user, &directory_ids)
            .map_err(|err| {
                eprintln!("Error generating JWT: {:?}", err);
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            })?;
        Ok(Json(token))
    } else {
        println!("User authentication failed");
        Err(axum::http::StatusCode::UNAUTHORIZED)
    }
}