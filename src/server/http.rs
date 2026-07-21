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
///
/// The fallback (`leptos_axum::file_and_error_handler` -> tower-http
/// `ServeDir`) percent-decodes the request path before resolving a file, so
/// the guard must decode too (e.g. `/%2eenv` decodes to `/.env`) or it can
/// be bypassed.
async fn dotfile_guard(req: Request, next: Next) -> Response {
    if is_dotfile_blocked(req.uri().path()) {
        return StatusCode::NOT_FOUND.into_response();
    }
    next.run(req).await
}

/// Pure decision function for [`dotfile_guard`]: percent-decodes `raw_path`
/// and reports whether it should be blocked. A malformed (non-UTF-8)
/// percent-encoding is blocked outright; otherwise a decoded path is
/// blocked when it contains a `/.` segment, unless it's under
/// `/.well-known/`.
fn is_dotfile_blocked(raw_path: &str) -> bool {
    percent_encoding::percent_decode_str(raw_path)
        .decode_utf8()
        .map_or(true, |decoded| {
            !decoded.starts_with("/.well-known/") && decoded.contains("/.")
        })
}

/// Path/response-based `Cache-Control`, mirroring nginx's two cache tiers:
/// content-hashed assets under `/pkg/` are immutable for a year; HTML
/// documents are never cached. A response that already carries its own
/// `Cache-Control` (e.g. `/version`) is left untouched.
///
/// A missing asset under `/pkg/` (e.g. `/pkg/missing.js`) falls through to
/// `file_and_error_handler`'s HTML error shell; only a *successful* `/pkg/`
/// response gets the immutable treatment, so that 404 shell instead falls
/// into the `is_html` no-cache branch below.
async fn cache_control(req: Request, next: Next) -> Response {
    let is_pkg_asset = req.uri().path().starts_with("/pkg/");
    let mut res = next.run(req).await;

    if res.headers().contains_key(header::CACHE_CONTROL) {
        return res;
    }

    let is_success = res.status().is_success();
    let is_html = res
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|content_type| content_type.starts_with("text/html"));

    if let Some(value) = cache_value_for(is_pkg_asset, is_success, is_html) {
        res.headers_mut()
            .insert(header::CACHE_CONTROL, HeaderValue::from_static(value));
    }

    res
}

/// Pure decision function for [`cache_control`]: picks the `Cache-Control`
/// value (if any) for a response, given whether it's a `/pkg/` asset,
/// whether the response succeeded, and whether it's HTML.
const fn cache_value_for(
    is_pkg_asset: bool,
    is_success: bool,
    is_html: bool,
) -> Option<&'static str> {
    if is_pkg_asset && is_success {
        Some("public, max-age=31536000, immutable")
    } else if is_html {
        Some("no-cache, no-store, must-revalidate")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{cache_value_for, is_dotfile_blocked};

    #[test]
    fn blocks_plain_dotfile() {
        assert!(is_dotfile_blocked("/.env"));
    }

    #[test]
    fn blocks_nested_dotfile() {
        assert!(is_dotfile_blocked("/foo/.git/config"));
    }

    #[test]
    fn blocks_percent_encoded_dotfile() {
        assert!(is_dotfile_blocked("/%2eenv"));
    }

    #[test]
    fn blocks_percent_encoded_nested_dotfile() {
        assert!(is_dotfile_blocked("/%2egit"));
    }

    #[test]
    fn blocks_percent_encoded_slash_before_dotfile() {
        assert!(is_dotfile_blocked("/foo%2f.env"));
    }

    #[test]
    fn blocks_malformed_percent_encoding() {
        assert!(is_dotfile_blocked("/%ff%fe"));
    }

    #[test]
    fn allows_well_known() {
        assert!(!is_dotfile_blocked("/.well-known/acme-challenge/x"));
    }

    #[test]
    fn allows_pkg_asset_path() {
        assert!(!is_dotfile_blocked("/pkg/app.js"));
    }

    #[test]
    fn allows_root() {
        assert!(!is_dotfile_blocked("/"));
    }

    #[test]
    fn pkg_success_is_immutable() {
        assert_eq!(
            cache_value_for(true, true, false),
            Some("public, max-age=31536000, immutable")
        );
    }

    #[test]
    fn pkg_404_html_is_no_cache() {
        assert_eq!(
            cache_value_for(true, false, true),
            Some("no-cache, no-store, must-revalidate")
        );
    }

    #[test]
    fn html_is_no_cache() {
        assert_eq!(
            cache_value_for(false, true, true),
            Some("no-cache, no-store, must-revalidate")
        );
    }

    #[test]
    fn other_is_none() {
        assert_eq!(cache_value_for(false, true, false), None);
    }
}
