use crate::entities::{
  ad_placement::{self, Entity as AdPlacement},
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
use sea_orm::{DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter, ActiveModelTrait};
use uuid::Uuid;
use chrono::{Utc, Duration};
use crate::auth::{hash_password, verify_password, generate_jwt, Claims};

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
) -> Result<Json<String>, StatusCode> {
    println!("Received login request for email: {} in directory: {}", login_data.email, login_data.directory_id);

    // Find the user by email
    let user = User::find()
        .filter(user::Column::Email.eq(&login_data.email))
        .one(&db)
        .await
        .map_err(|err| {
            eprintln!("Error fetching user: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify password
    if verify_password(&login_data.password, &user.password_hash)
        .map_err(|err| {
            eprintln!("Error verifying password: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    {
        println!("User authenticated");

        // Fetch user's profile for the specified directory
        let user_profile = user_profile::Entity::find()
            .filter(user_profile::Column::UserId.eq(user.id))
            .find_with_related(profile::Entity)
            .all(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .into_iter()
            .find(|(_, profiles)| profiles.first().map_or(false, |p| p.directory_id == login_data.directory_id));

        if let Some((user_profile, profile)) = user_profile {
            let profile = profile.first().unwrap(); // Safe because we checked in the find() above
            
            // Generate JWT including user_id, profile_id, and directory_id
            let claims = Claims {
                sub: user.id.to_string(),
                profile_id: user_profile.profile_id.to_string(),
                directory_id: profile.directory_id.to_string(),
                exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
            };

            let token = generate_jwt(claims)
                .map_err(|err| {
                    eprintln!("Error generating JWT: {:?}", err);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

            Ok(Json(token))
        } else {
            println!("User not associated with the specified directory");
            Err(StatusCode::FORBIDDEN)
        }
    } else {
        println!("User authentication failed");
        Err(StatusCode::UNAUTHORIZED)
    }
}