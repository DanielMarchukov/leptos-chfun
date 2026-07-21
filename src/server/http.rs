use axum::{
    extract::Request,
    http::{header, HeaderName, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use leptos::prelude::LeptosOptions;
use tower_http::{compression::CompressionLayer, set_header::SetResponseHeaderLayer};

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

/// nginx-parity middleware: everything `nginx.conf` used to provide at the
/// reverse-proxy layer, reproduced as `tower`/axum middleware.
///
/// Sets security headers, gzip-compresses responses, applies path-based
/// `Cache-Control`, and guards dotfile paths. Wraps the *whole* router
/// (including the `/pkg` static-file and SPA fallback), so callers should
/// apply it last, after every route, merge, and `.fallback()` call.
///
/// Layer order (outermost to innermost, i.e. request flow top to bottom):
/// security headers -> compression -> dotfile guard -> cache-control ->
/// routes. Security headers sit outermost so they land on every response,
/// including a 404 from the dotfile guard; cache-control sits innermost so
/// it can inspect the real response a handler produced.
pub fn apply_middleware<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    router
        .layer(middleware::from_fn(cache_control))
        .layer(middleware::from_fn(dotfile_guard))
        .layer(CompressionLayer::new())
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::REFERRER_POLICY,
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("permissions-policy"),
            HeaderValue::from_static("geolocation=(), microphone=(), camera=()"),
        ))
}

/// Refuse any request for a hidden (dot-prefixed) path segment, e.g.
/// `/.env` or `/foo/.git/config`, mirroring nginx's
/// `location ~ /\.(?!well-known)`. `/.well-known/...` stays reachable, e.g.
/// for ACME challenges or `security.txt`.
async fn dotfile_guard(req: Request, next: Next) -> Response {
    let path = req.uri().path();
    if !path.starts_with("/.well-known/") && path.contains("/.") {
        return StatusCode::NOT_FOUND.into_response();
    }
    next.run(req).await
}

/// Path/response-based `Cache-Control`, mirroring nginx's two cache tiers:
/// content-hashed assets under `/pkg/` are immutable for a year; HTML
/// documents are never cached. A response that already carries its own
/// `Cache-Control` (e.g. `/version`) is left untouched.
async fn cache_control(req: Request, next: Next) -> Response {
    let is_pkg_asset = req.uri().path().starts_with("/pkg/");
    let mut res = next.run(req).await;

    if res.headers().contains_key(header::CACHE_CONTROL) {
        return res;
    }

    let is_html = res
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|content_type| content_type.starts_with("text/html"));

    let cache_control = if is_pkg_asset {
        Some("public, max-age=31536000, immutable")
    } else if is_html {
        Some("no-cache, no-store, must-revalidate")
    } else {
        None
    };

    if let Some(value) = cache_control {
        res.headers_mut()
            .insert(header::CACHE_CONTROL, HeaderValue::from_static(value));
    }

    res
}
