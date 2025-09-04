use crate::{
    dto::MyErr,
    utils::{app_state::AppState, jwt::Claim},
};
use axum::{
    extract::{Request, State},
    middleware::Next,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, decode};

pub async fn authorizer(
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<axum::http::Response<axum::body::Body>, MyErr> {
    req.extensions_mut().insert(
        decode::<Claim>(
            token.token(),
            &DecodingKey::from_secret(state.secret.as_ref().as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| MyErr::Unauthorized("unauthorized_user".to_string()))?
        .claims,
    );
    Ok(next.run(req).await)
}

pub async fn giving_delay(req: Request, next: Next) -> axum::http::Response<axum::body::Body> {
    // tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    next.run(req).await
}
