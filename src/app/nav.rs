use leptos::prelude::*;

use crate::app::{IconInstagram, IconMenu, IconX};

/// Sticky top nav (Figma `3:8`), mobile menu via a signal + `<Show>`, SSR
/// closed by default; the two `<nav>`s carry distinct aria-labels. See README.
#[allow(clippy::must_use_candidate)]
#[component]
pub fn Nav() -> impl IntoView {
    let (open, set_open) = signal(false);
    let instagram_href = crate::app::INSTAGRAM_URL;

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
                    href=instagram_href
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
                    href=instagram_href
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
