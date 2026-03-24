use crate::{
    entity::problems,
    error::AppError,
    problem::dto::UpdateProblemPayload,
    utils::{
        app_state::AppState,
        helpers::{SetFromOption, SetFromValue},
        security::Claim,
    },
};
use axum::{Extension, Json, debug_handler, extract::State};
use sea_orm::{ColumnTrait, EntityTrait, ExprTrait, QueryFilter};
use serde_json::{Value, json};

#[debug_handler]
pub async fn update(
    State(stt): State<AppState>,
    Extension(claim): Extension<Claim>,
    Json(payload): Json<UpdateProblemPayload>,
) -> Result<Json<Value>, AppError> {
    let actv_mdl = problems::ActiveModel {
        title: payload.title.set_from_opt(),
        slug: payload.slug.set_from_opt(),
        statement: payload.statement.set(),
        input_spec: payload.input_spec.set(),
        output_spec: payload.output_spec.set(),
        sample_inputs: payload.sample_inputs.map(|s| Value::String(s)).set(),
        sample_outputs: payload.sample_outputs.map(|s| Value::String(s)).set(),
        time_limit: payload.time_limit.set_from_opt(),
        memory_limit: payload.memory_limit.set_from_opt(),
        difficulty: payload.difficulty.set(),
        is_public: payload.is_public.set_from_opt(),
        created_at: payload.created_at.set_from_opt(),
        author_id: payload.author_id.set(),
        ..Default::default()
    };
    let rows_affected = problems::Entity::update_many()
        .filter(
            problems::Column::Id
                .eq(payload.id)
                .and(problems::Column::AuthorId.eq(claim.id)),
        )
        .set(actv_mdl)
        .exec(stt.db.as_ref())
        .await
        .map_err(|e| AppError::internal(e.to_string()))?
        .rows_affected;

    Ok(Json::from(json!({
        "status": 200,
        "msg": format!("Updated the problem, Res: {}", rows_affected),
    })))
}
