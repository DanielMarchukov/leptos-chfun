use leptos::prelude::*;

use crate::app::{IconInstagram, IconMenu, IconX};

/// Instagram handle used across the site (nav, footer, Instagram section) —
/// `docs/frontend.md` §8 "Follow the Flow" names the handle pill
/// `@ch.pilatesfun`.
const INSTAGRAM_HANDLE: &str = "ch.pilatesfun";

/// Sticky top nav — `docs/frontend.md` §1 (`3:8`, h=94).
///
/// The mobile menu is the first hydration-interactive piece in the app: a
/// `signal` toggled by the hamburger button, rendered via `<Show>`. SSR
/// always renders the closed state; hydration wakes the toggle.
///
/// Anchor contract for later sections (Tasks 5-8 add the matching `id`s):
/// `#top` (page top / hero), `#about`, `#gallery`, `#services`, `#connect`.
///
/// a11y: the two `<nav>` landmarks carry distinguishing `aria-label`s
/// ("Primary" / "Mobile") so a screen reader doesn't announce two unlabeled
/// navigations, and the hamburger's `aria-label` tracks `open` ("Open menu" /
/// "Close menu") alongside its `aria-expanded` state.
#[allow(clippy::must_use_candidate)]
#[component]
pub fn Nav() -> impl IntoView {
    let (open, set_open) = signal(false);
    let instagram_href = format!("https://instagram.com/{INSTAGRAM_HANDLE}");

    let close = move |_| set_open.set(false);

    view! {
        <header class="sticky top-0 z-50 flex h-[94px] items-center justify-between gap-4 bg-cream/80 px-6 backdrop-blur md:px-[120px]">
            <a
                href="#top"
                class="flex min-w-0 flex-wrap items-baseline gap-x-1 font-display text-xl text-ink md:text-[26px]"
            >
                "Chean Hui Toh"
                <span class="text-sm text-ink/60 md:text-[18px]">" / Pilates"</span>
            </a>

            <nav aria-label="Primary" class="hidden items-center gap-12 text-[15px] text-ink md:flex">
                <a href="#about">"About"</a>
                <a href="#gallery">"Gallery"</a>
                <a href="#services">"Services"</a>
                <a href="#connect">"Connect"</a>
                <a
                    href=instagram_href.clone()
                    target="_blank"
                    rel="noopener noreferrer"
                    aria-label="Instagram"
                >
                    <IconInstagram class="w-5 h-5".to_string() />
                </a>
            </nav>

            <button
                type="button"
                class="flex h-11 w-11 shrink-0 items-center justify-center text-ink md:hidden"
                aria-label=move || if open.get() { "Close menu" } else { "Open menu" }
                aria-expanded=move || open.get().to_string()
                on:click=move |_| set_open.update(|o| *o = !*o)
            >
                <Show when=move || open.get() fallback=|| view! { <IconMenu /> }>
                    <IconX />
                </Show>
            </button>
        </header>

        <Show when=move || open.get()>
            <nav aria-label="Mobile" class="flex flex-col gap-1 bg-cream px-6 pb-6 text-ink md:hidden">
                <a class="flex min-h-11 items-center" href="#about" on:click=close>
                    "About"
                </a>
                <a class="flex min-h-11 items-center" href="#gallery" on:click=close>
                    "Gallery"
                </a>
                <a class="flex min-h-11 items-center" href="#services" on:click=close>
                    "Services"
                </a>
                <a class="flex min-h-11 items-center" href="#connect" on:click=close>
                    "Connect"
                </a>
                <a
                    class="flex min-h-11 items-center gap-2"
                    href=instagram_href.clone()
                    target="_blank"
                    rel="noopener noreferrer"
                    aria-label="Instagram"
                >
                    <IconInstagram class="w-5 h-5".to_string() />
                    "Instagram"
                </a>
            </nav>
        </Show>
    }
}
