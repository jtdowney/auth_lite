use std::net::SocketAddr;

use axum::{
    extract::State,
    headers::{authorization::Basic, Authorization},
    http::{header, HeaderMap, Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router, TypedHeader,
};
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level};

use crate::auth;

async fn auth(
    State(pool): State<sqlx::SqlitePool>,
    TypedHeader(auth): TypedHeader<Authorization<Basic>>,
) -> StatusCode {
    let username = auth.username();
    let password = auth.password();
    match auth::authenticate(&pool, username, password).await {
        Ok(valid) => {
            if valid {
                StatusCode::OK
            } else {
                StatusCode::UNAUTHORIZED
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn ensure_authentication<B>(
    headers: header::HeaderMap,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, HeaderMap)> {
    if headers.contains_key(header::AUTHORIZATION) {
        let response = next.run(request).await;
        Ok(response)
    } else {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::WWW_AUTHENTICATE,
            header::HeaderValue::from_static("Basic realm=\"auth-lite\""),
        );

        Err((StatusCode::UNAUTHORIZED, headers))
    }
}

pub fn app(pool: sqlx::SqlitePool) -> Router {
    Router::new()
        .route("/auth", get(auth))
        .layer(middleware::from_fn(ensure_authentication))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros),
                ),
        )
        .with_state(pool)
}

pub async fn start(addr: SocketAddr, pool: sqlx::SqlitePool) -> anyhow::Result<()> {
    info!("Starting server on {}", addr);

    let app = app(pool);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
