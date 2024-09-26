use sea_orm::{DatabaseConnection,ColumnTrait, EntityTrait, Set, QueryFilter};
use crate::entities::user;
use crate::auth::hash_password;
use uuid::Uuid;
use chrono::Utc;

pub async fn create_admin_user_if_not_exists(
    db: &DatabaseConnection,
    email: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the admin user already exists
    let existing_admin = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await?;
    if existing_admin.is_none() {
        // Create the admin user
        let hashed_password = hash_password(password)?;
        let new_admin = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set("admin".to_string()),
            email: Set(email.to_string()),
            password_hash: Set(hashed_password),
            is_admin: Set(true),
            is_active: Set(true),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };

        user::Entity::insert(new_admin).exec(db).await?;
        tracing::info!("Admin user created successfully");
    } else {
        println!("Admin Found");
        tracing::info!("Admin user already exists");
    }

    Ok(())
}
