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
    match Business::find().all(&db).await {
        Ok(businesses) => Ok(Json(businesses)),
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
    let hashed_password = hash_password(&user_data.password)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_user = user::ActiveModel {
        username: Set(user_data.username),
        email: Set(user_data.email),
        password_hash: Set(hashed_password),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };

    match User::insert(new_user).exec(&db).await {
        Ok(result) => {
            let user = User::find_by_id(result.last_insert_id)
                .one(&db)
                .await
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(user))
        },
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn login_user(
    State(db): State<DatabaseConnection>,
    Json(login_data): Json<UserLogin>,
) -> Result<Json<String>, axum::http::StatusCode> {
    let user = User::find()
        .filter(user::Column::Email.eq(&login_data.email))
        .one(&db)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    if verify_password(&login_data.password, &user.password_hash)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let token = generate_jwt(&user)
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(token))
    } else {
        Err(axum::http::StatusCode::UNAUTHORIZED)
    }
}
