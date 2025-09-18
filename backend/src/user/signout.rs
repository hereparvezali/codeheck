use tower_cookies::{Cookie, Cookies};

pub async fn signout(cookies: Cookies) -> &'static str {
    cookies.remove(
        Cookie::build("refresh_token")
            .path("/")
            .http_only(true)
            .build(),
    );
    "logout_successful"
}
