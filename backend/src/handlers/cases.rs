use uuid::Uuid;
use chrono::Utc;
use sea_orm::{
    IntoActiveModel,
    ActiveModelTrait, 
    Set, 
    DatabaseConnection,DbErr, 
    EntityTrait, 
    QueryFilter, 
    ColumnTrait};
use crate::entities::{activity, case};

pub async fn create_case_activity(
    db: &DatabaseConnection,
    case_id: Uuid,
    account_id: Uuid,
    title: String,
    description: Option<String>,
    created_by: Uuid,
) -> Result<activity::Model, DbErr> {
    let mut activity = activity::ActiveModel {
        id: Set(Uuid::new_v4()),
        account_id: Set(account_id),
        activity_type: Set(activity::ActivityType::Note),
        title: Set(title),
        description: Set(description),
        status: Set(activity::ActivityStatus::Completed),
        created_by: Set(created_by),
        associated_entities: Set(serde_json::Value::Array(vec![])),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    }.insert(db).await?;

    // Associate the activity with the case
    let case_association = activity::AssociatedEntity {
        entity_type: activity::AssociatedEntityType::Case,
        entity_id: case_id,
    };
    activity.add_associated_entity(case_association).map_err(|e| sea_orm::DbErr::Custom(e.to_string()))?;

    // If you want to associate with the account as well
    let account_association = activity::AssociatedEntity {
        entity_type: activity::AssociatedEntityType::Account,
        entity_id: account_id,
    };
    activity.add_associated_entity(account_association)
        .map_err(|e| sea_orm::DbErr::Custom(e.to_string()))?;

    // Update the activity in the database with the new associations
    activity::Entity::update(activity.clone().into_active_model())
        .filter(activity::Column::Id.eq(activity.id))
        .exec(db)
        .await?;

    Ok(activity)
}

pub async fn get_case_activities(
    db: &DatabaseConnection,
    case_id: Uuid,
) -> Result<Vec<activity::Model>, DbErr> {
    activity::Entity::find()
        .filter(
            activity::Column::AssociatedEntities.contains(
                serde_json::json!({
                    "entity_type": "Case",
                    "entity_id": case_id.to_string()
                }).to_string()
            )
        )
        .all(db)
        .await
}
