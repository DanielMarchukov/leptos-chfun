//! Shared isomorphic types crossing the `get_feed` server-fn boundary (not
//! ssr-gated, so both `hydrate` and `ssr` builds compile them). See README
//! (Features: Instagram feed contract).

use serde::{Deserialize, Serialize};

/// One Instagram post rendered in the "Follow the Flow" grid. Serializes
/// SSR→hydrate via the `get_feed` `Resource`. See README (Features).
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

/// The kind of an Instagram post — reels get a play overlay in the grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeedKind {
    /// A still image (`IMAGE` / `CAROUSEL_ALBUM`).
    Photo,
    /// A reel (`VIDEO` + `REELS`) — rendered with a play glyph over the poster.
    Reel,
}
