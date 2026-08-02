use leptos::prelude::*;

use crate::app::IconInstagram;
use crate::models::{FeedItem, FeedKind};

/// Public Instagram profile URL — linked from the handle pill and CTA.
const INSTAGRAM_URL: &str = "https://instagram.com/ch.pilatesfun";

/// Instagram feed source — Phase-4 stub returning `[]` (section shows its static
/// fallback). Shared ssr/hydrate module so hydrate can resolve the symbol.
// #[server] fns are always async; Phase 4's body will await the cache read.
#[allow(clippy::unused_async)]
#[server(GetFeed)]
pub async fn get_feed() -> Result<Vec<FeedItem>, ServerFnError> {
    Ok(vec![])
}

/// Instagram — "Follow the Flow" (Figma `3:131`), `id="connect"`. `get_feed`
/// Resource → Suspense/Show → static fallback (empty stub). See README.
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
                <a
                    href=INSTAGRAM_URL
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex items-center gap-2.5 rounded-full border border-ink/10 bg-white px-4 py-2.5 text-ink shadow-[0_6px_9px_rgba(0,0,0,0.05)]"
                >
                    <IconInstagram class="w-[18px] h-[18px]".to_string() />
                    <span class="font-sans text-lg font-bold">"@ch.pilatesfun"</span>
                </a>
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

/// One real-feed tile — image linking to its `permalink`, with a play glyph
/// for reels. Wired for Phase 4; unexercised while the stub yields `[]`.
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

/// Static fallback grid — six tiles alternating `sage-tint`/`blush` with a
/// muted `IconInstagram`. Shown while pending or empty (always, per the stub).
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
