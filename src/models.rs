//! Shared, isomorphic types that cross the server-fn boundary (e.g. `FeedItem`).
//!
//! Kept here (not ssr-gated) so both the client (`hydrate`) and server (`ssr`)
//! builds compile them — the Instagram grid reads them across the `get_feed`
//! server-function boundary, and a Leptos `Resource` must serialize its value
//! from SSR to the hydrating client. The Phase-4 backend
//! (`docs/instagram.md` §2) maps raw `IgMedia` into these; until then the
//! `get_feed` stub returns an empty feed and the grid renders its fallback.

use serde::{Deserialize, Serialize};

/// One Instagram post rendered in the "Follow the Flow" grid.
///
/// The `id`-stable `image_path` is a **local** copy served from `/media/ig/`
/// (Phase 4 downloads the bytes so the grid survives Instagram's expiring CDN
/// URLs); `permalink` is the public instagram.com link the tile clicks out to.
///
/// Derives `Serialize`/`Deserialize` because the value crosses the server-fn
/// boundary (SSR serializes the `Resource` value into the page, the client
/// deserializes it on hydrate); `Eq` accompanies `PartialEq` to satisfy
/// clippy's `derive_partial_eq_without_eq` (every field is itself `Eq`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedItem {
    /// Instagram media id — the `<For>` key and the `<id>.jpg` cache filename.
    pub id: String,
    /// Photo vs. reel — a reel tile gets a play affordance overlaid.
    pub kind: FeedKind,
    /// Local, self-hosted image path, e.g. `/media/ig/<id>.jpg`.
    pub image_path: String,
    /// Public instagram.com URL the tile links out to.
    pub permalink: String,
    /// Accessible alt text, derived from the caption (truncated/sanitised).
    pub alt: String,
    /// ISO-8601 post timestamp from the Graph API.
    pub timestamp: String,
}

/// The kind of an Instagram post.
///
/// Reels are `media_type=VIDEO` + `media_product_type=REELS` upstream
/// (`docs/instagram.md` §1); the grid uses the poster image for both and only
/// differs by the play overlay.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeedKind {
    /// A still image (`IMAGE` / `CAROUSEL_ALBUM`).
    Photo,
    /// A reel (`VIDEO` + `REELS`) — rendered with a play glyph over the poster.
    Reel,
}
