use axum::{http::header, response::IntoResponse, routing::get, Router};
use leptos::prelude::LeptosOptions;

/// `/healthz` (liveness) and `/version` (the build's `GIT_SHA`, no-cache).
/// The deploy pipeline polls /version for an exact github.sha match.
pub fn status_routes() -> Router<LeptosOptions> {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/version", get(version))
}

async fn healthz() -> &'static str {
    "ok\n"
}

async fn version() -> impl IntoResponse {
    let sha = option_env!("GIT_SHA").unwrap_or("unknown");
    (
        [
            (header::CONTENT_TYPE, "text/plain; charset=utf-8"),
            (header::CACHE_CONTROL, "no-cache, no-store, must-revalidate"),
        ],
        sha,
    )
}
