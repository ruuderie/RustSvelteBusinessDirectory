use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use sea_orm::{DatabaseConnection, EntityTrait, Condition, ColumnTrait, QueryFilter};
use crate::entities::business::{self, Entity as Business};
use crate::models::BusinessSearch;

pub fn router() -> Router<DatabaseConnection> {
    Router::new()
        .route("/businesses", get(get_businesses))
        .route("/businesses/search", get(search_businesses))
        .route("/businesses/:id", get(get_business_by_id))
}

async fn get_businesses(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<business::Model>>, axum::http::StatusCode> {
    println!("Fetching all businesses");
    match Business::find().all(&db).await {
        Ok(businesses) => {
            println!("Found {} businesses", businesses.len());
            Ok(Json(businesses))
        },
        Err(e) => {
            eprintln!("Error fetching businesses: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn search_businesses(
    State(db): State<DatabaseConnection>,
    Query(query): Query<BusinessSearch>,
) -> Result<Json<Vec<business::Model>>, axum::http::StatusCode> {
    Business::find()
        .filter(
            Condition::any()
                .add(business::Column::Name.contains(&query.q))
                .add(business::Column::Category.contains(&query.q))
        )
        .all(&db)
        .await
        .map(Json)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_business_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<Option<business::Model>>, axum::http::StatusCode> {
    Business::find_by_id(id)
        .one(&db)
        .await
        .map(Json)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}
