use auth_lite::{auth, server};
use axum::{
    body::Body,
    http::{
        header::{AUTHORIZATION, WWW_AUTHENTICATE},
        HeaderValue, Request, StatusCode,
    },
};
use base64::{engine::general_purpose::STANDARD as Base64, Engine};
use tower::ServiceExt;

mod support;

#[tokio::test]
async fn successful_auth_request() -> anyhow::Result<()> {
    let (pool, _db_file) = support::create_test_db().await?;
    auth::create_user(&pool, "test-user", "test-password").await?;

    let app = server::app(pool.clone());
    let request = Request::builder()
        .uri("/auth")
        .header(
            AUTHORIZATION,
            format!("Basic {}", Base64.encode("test-user:test-password")),
        )
        .body(Body::empty())?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn failed_auth_request_missing_user() -> anyhow::Result<()> {
    let (pool, _db_file) = support::create_test_db().await?;

    let app = server::app(pool.clone());
    let request = Request::builder()
        .uri("/auth")
        .header(
            AUTHORIZATION,
            format!("Basic {}", Base64.encode("test-user:test-password")),
        )
        .body(Body::empty())?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    Ok(())
}

#[tokio::test]
async fn failed_auth_request_bad_password() -> anyhow::Result<()> {
    let (pool, _db_file) = support::create_test_db().await?;
    auth::create_user(&pool, "test-user", "test-password").await?;

    let app = server::app(pool.clone());
    let request = Request::builder()
        .uri("/auth")
        .header(
            AUTHORIZATION,
            format!("Basic {}", Base64.encode("test-user:bad-password")),
        )
        .body(Body::empty())?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    Ok(())
}

#[tokio::test]
async fn failed_auth_no_header() -> anyhow::Result<()> {
    let (pool, _db_file) = support::create_test_db().await?;

    let app = server::app(pool.clone());
    let request = Request::builder().uri("/auth").body(Body::empty())?;
    let response = app.oneshot(request).await?;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        response.headers().get(WWW_AUTHENTICATE).unwrap(),
        HeaderValue::from_static("Basic realm=\"auth-lite\"")
    );

    Ok(())
}
