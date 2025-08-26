use axum::{
    Extension, Json,
    extract::{Query, State},
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::{
    dto::MyErr,
    entity::problems,
    utils::{app_state::AppState, jwt::Claim},
};

use super::dto::RetrieveProblemQuery;

pub async fn retrieve(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
    Query(query): Query<RetrieveProblemQuery>,
) -> Result<Json<problems::Model>, MyErr> {
    Ok(Json(
        problems::Entity::find()
            .filter(
                Condition::all()
                    .add(
                        Condition::any()
                            .add_option(query.id.map(|id| problems::Column::Id.eq(id)))
                            .add_option(query.slug.map(|slug| problems::Column::Slug.eq(slug))),
                    )
                    .add(
                        problems::Column::IsPublic
                            .eq(true)
                            .or(problems::Column::AuthorId.eq(claim.id)),
                    ),
            )
            .one(stt.db.as_ref())
            .await
            .map_err(|e| MyErr::InternalServerErrorWithMessage(e.to_string()))?
            .ok_or(MyErr::NotFound("not_available".to_string()))?,
    ))
}
