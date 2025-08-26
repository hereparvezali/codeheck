use tower_cookies::{Cookie, Cookies};

pub async fn signout(cookies: Cookies) -> &'static str {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    cookies.remove(
        Cookie::build("refresh_token")
            .path("/")
            .http_only(true)
            .build(),
    );
    "logout_successful"
}
