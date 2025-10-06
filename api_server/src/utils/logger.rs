use axum::{
    extract::Request,
    http::{Method, Uri},
    middleware::Next,
    response::Response,
};
use std::time::Instant;

/// Simple logging middleware that logs request method, URI, and duration
pub async fn logger(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status();

    log_request(method, uri, status, duration);

    response
}

fn log_request(
    method: Method,
    uri: Uri,
    status: axum::http::StatusCode,
    duration: std::time::Duration,
) {
    let status_emoji = if status.is_success() {
        "✅"
    } else if status.is_client_error() {
        "⚠️"
    } else if status.is_server_error() {
        "❌"
    } else {
        "ℹ️"
    };

    println!(
        "{} {} {} {} - {:?}",
        status_emoji, method, uri, status, duration
    );
}
