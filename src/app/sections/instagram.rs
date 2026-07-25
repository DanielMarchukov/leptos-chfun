use leptos::prelude::*;

use crate::app::IconInstagram;
use crate::models::{FeedItem, FeedKind};

/// Public Instagram profile for @ch.pilatesfun — the grid header handle pill
/// and the "Follow on Instagram" CTA both link here (matches `nav.rs`).
const INSTAGRAM_URL: &str = "https://instagram.com/ch.pilatesfun";

/// Instagram feed source — **Phase 4 stub**.
///
/// This is the only `get_feed()`-driven surface in the app. It is deliberately
/// co-located with the sole UI that calls it and lives in a module compiled
/// for BOTH `ssr` and `hydrate`: the `#[server]` macro emits the server body
/// under `ssr` and the client-side call otherwise, so the symbol must exist in
/// the hydrate build too. Phase 4 replaces the body with a read of the
/// server-side feed cache (`docs/instagram.md` §4); until then it returns an
/// empty feed, so the section renders its static fallback tiles. `Ok(vec![])`
/// plus the caller's `.unwrap_or_default()` means even a failed call degrades
/// to `[]` → fallback, never a panic.
// `async` with no `.await` is required by the `#[server]` contract (server
// functions are always async) and Phase 4's body will await the cache read;
// clippy's `unused_async` can't see that future obligation.
#[allow(clippy::unused_async)]
#[server(GetFeed)]
pub async fn get_feed() -> Result<Vec<FeedItem>, ServerFnError> {
    Ok(vec![])
}

/// Instagram — "Follow the Flow" — `docs/frontend.md` §8 (Figma `3:131`).
///
/// `bg-cream`, centered: an H2 + a rounded white **handle pill**
/// (`IconInstagram` + `@ch.pilatesfun`), then the `Resource`-driven grid, then
/// a dark **"Follow on Instagram"** pill CTA. `id="connect"` is the Nav
/// "Connect" anchor target.
///
/// The grid binds to [`get_feed`] via a `Resource` (its `Vec<FeedItem>` value
/// serializes from SSR into the page and deserializes on hydrate — no client
/// refetch). `<Suspense>` covers the pending state, then `<Show>` swaps between
/// the real `<For>` grid (non-empty feed) and the static fallback (empty). With
/// the Phase-4 stub returning `[]`, the fallback always renders; the `<For>`
/// branch (including the `FeedKind::Reel` play overlay) is wired for Phase 4.
///
/// A direct `get_design_context` pull on `3:131` supplied the exact treatment:
/// section gap `64px`, header gap `10px`; H2 "Follow the Flow" (the design tags
/// it Fraunces SemiBold 40px — rendered here as plain `font-display` for weight
/// parity with the other section H2s, which the type scale fixes at Fraunces
/// 400; flagged, not silently thickened); handle pill `bg-white` + `18px` bold
/// Geist + a soft `0 6px 9px rgba(0,0,0,.05)` shadow; a `368px` 3-col grid at
/// gap `16px` over two rows with `rounded-[24px]` tiles; the CTA `bg-ink` white
/// SemiBold 16px pill at `px-28 py-16` with a `0 10px 12px rgba(0,0,0,.1)`
/// shadow. Responsive: the grid is `grid-cols-2 md:grid-cols-3` with
/// `aspect-square` tiles, header and CTA stay centered.
#[allow(clippy::must_use_candidate)]
#[component]
pub fn InstagramSection() -> impl IntoView {
    let feed = Resource::new(|| (), |()| async { get_feed().await.unwrap_or_default() });
    view! {
        <section
            id="connect"
            class="flex flex-col items-center gap-10 bg-cream px-6 py-16 md:gap-16 md:px-[120px] md:py-[160px]"
        >
            <div class="flex flex-col items-center gap-2.5">
                <h2 class="font-display text-[clamp(2rem,5vw,40px)] leading-[1.1] text-ink">
                    "Follow the Flow"
                </h2>
                <div class="flex items-center gap-2.5 rounded-full border border-ink/10 bg-white px-4 py-2.5 text-ink shadow-[0_6px_9px_rgba(0,0,0,0.05)]">
                    <IconInstagram class="w-[18px] h-[18px]".to_string() />
                    <span class="font-sans text-lg font-bold">"@ch.pilatesfun"</span>
                </div>
            </div>

            <Suspense fallback=|| view! { <IgFallbackGrid /> }>
                {move || {
                    feed.get()
                        .map(|items| {
                            view! {
                                <Show
                                    when={
                                        let items = items.clone();
                                        move || !items.is_empty()
                                    }
                                    fallback=|| view! { <IgFallbackGrid /> }
                                >
                                    <div class="grid w-full max-w-[1200px] grid-cols-2 gap-4 md:grid-cols-3">
                                        <For
                                            each={
                                                let items = items.clone();
                                                move || items.clone()
                                            }
                                            key=|item| item.id.clone()
                                            children=ig_tile
                                        />
                                    </div>
                                </Show>
                            }
                        })
                }}
            </Suspense>

            <a
                href=INSTAGRAM_URL
                target="_blank"
                rel="noopener noreferrer"
                class="flex items-center gap-2.5 rounded-full bg-ink px-7 py-4 text-white shadow-[0_10px_12px_rgba(0,0,0,0.1)]"
            >
                <IconInstagram class="w-[18px] h-[18px]".to_string() />
                <span class="font-sans text-base font-semibold">"Follow on Instagram"</span>
            </a>
        </section>
    }
}

/// One real-feed tile — an image linking out to its `permalink`, with a play
/// glyph overlaid for reels.
///
/// Passed as `<For>`'s `children`; exercised only once Phase 4 supplies a
/// non-empty feed (the stub yields `[]`), but wired and type-checked now so the
/// Phase-4 swap touches no UI.
fn ig_tile(item: FeedItem) -> impl IntoView {
    let is_reel = matches!(item.kind, FeedKind::Reel);
    view! {
        <a
            href=item.permalink
            target="_blank"
            rel="noopener noreferrer"
            class="relative block aspect-square overflow-hidden rounded-[24px]"
        >
            <img src=item.image_path alt=item.alt class="size-full object-cover" />
            <Show when=move || is_reel>
                <span class="absolute right-3 top-3 flex h-7 w-7 items-center justify-center rounded-full bg-black/40 text-white">
                    <svg
                        class="w-4 h-4"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                        aria-hidden="true"
                    >
                        <path d="M8 5v14l11-7z" />
                    </svg>
                </span>
            </Show>
        </a>
    }
}

/// Static fallback grid — six square tinted tiles matching the design's 2×3
/// layout.
///
/// Tiles alternate `sage-tint` / `blush` and each centers a muted
/// `IconInstagram`. Rendered while the feed is pending (Suspense) and when it
/// is empty (Show), which — with the Phase-4 stub — is always. Uses only design
/// tokens and needs no new image assets, so it reads as a calm "feed
/// placeholder" rather than a broken grid.
#[allow(clippy::must_use_candidate)]
#[component]
fn IgFallbackGrid() -> impl IntoView {
    view! {
        <div class="grid w-full max-w-[1200px] grid-cols-2 gap-4 md:grid-cols-3">
            {(0..6)
                .map(|i| {
                    let fill = if i % 2 == 0 { "bg-sage-tint" } else { "bg-blush" };
                    view! {
                        <div class=format!(
                            "flex aspect-square items-center justify-center rounded-[24px] text-ink/30 {fill}",
                        )>
                            <IconInstagram class="w-10 h-10".to_string() />
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}
