use super::dto::RetrieveUserResponse;
use crate::{
    dto::MyErr, entity::users, user::dto::RetrieveUserinfoQuery, utils::app_state::AppState,
};
use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, sea_query::Cond};

pub async fn retrieve(
    State(stt): State<AppState>,
    Query(query): Query<RetrieveUserinfoQuery>,
) -> Result<Json<RetrieveUserResponse>, MyErr> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok(Json(
        users::Entity::find()
            .filter(
                Cond::any()
                    .add_option(query.id.map(|id| users::Column::Id.eq(id)))
                    .add_option(
                        query
                            .username
                            .map(|username| users::Column::Username.eq(username)),
                    )
                    .add_option(query.email.map(|email| users::Column::Email.eq(email))),
            )
            .one(stt.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?
            .ok_or(MyErr::NotFound("user_not_found".to_string()))
            .map(|usr| {
                RetrieveUserResponse::new(usr.username, usr.email, usr.rating, usr.created_at)
            })?,
    ))
}
