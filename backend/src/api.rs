use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use sea_orm::{DatabaseConnection, EntityTrait, Condition, ColumnTrait, QueryFilter};
use crate::entities::business::{self, Entity as Business};
use crate::models::BusinessSearch;
use std::time::Instant;

pub fn router() -> Router<DatabaseConnection> {
    Router::new()
        .route("/businesses", get(get_businesses))
        .route("/businesses/search", get(search_businesses))
        .route("/businesses/:id", get(get_business_by_id))
}

async fn get_businesses(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<business::Model>>, axum::http::StatusCode> {
    let start = Instant::now();
    println!("Fetching all businesses");
    match Business::find().all(&db).await {
        Ok(businesses) => {
            println!("Found {} businesses in {:?}", businesses.len(), start.elapsed());
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
    let start = Instant::now();
    println!("Searching businesses with query: {}", query.q);
    let result = Business::find()
        .filter(
            Condition::any()
                .add(business::Column::Name.contains(&query.q))
                .add(business::Column::Category.contains(&query.q))
        )
        .all(&db)
        .await;
    
    match result {
        Ok(businesses) => {
            println!("Found {} businesses matching '{}' in {:?}", businesses.len(), query.q, start.elapsed());
            Ok(Json(businesses))
        },
        Err(e) => {
            eprintln!("Error searching businesses: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_business_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<Option<business::Model>>, axum::http::StatusCode> {
    let start = Instant::now();
    println!("Fetching business with id: {}", id);
    let result = Business::find_by_id(id).one(&db).await;
    
    match result {
        Ok(business) => {
            match &business {
                Some(_) => println!("Found business with id {} in {:?}", id, start.elapsed()),
                None => println!("No business found with id {} (query took {:?})", id, start.elapsed()),
            }
            Ok(Json(business))
        },
        Err(e) => {
            eprintln!("Error fetching business by id: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
