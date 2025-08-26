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
    // println!("{:?}", token.token());
    let myclaim: Claim = decode(
        token.token(),
        &DecodingKey::from_secret(state.secret.as_ref().as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| MyErr::Unauthorized("unauthorized_user".to_string()))?
    .claims;
    req.extensions_mut().insert(myclaim);

    Ok(next.run(req).await)
}
