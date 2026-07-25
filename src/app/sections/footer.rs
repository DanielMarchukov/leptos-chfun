use leptos::prelude::*;

use crate::app::{IconInstagram, IconLinkedIn};

/// Public Instagram profile for @ch.pilatesfun — the footer social icon links
/// out here (matches `nav.rs` / the Instagram section).
const INSTAGRAM_URL: &str = "https://instagram.com/ch.pilatesfun";

/// Footer — `docs/frontend.md` §9 (Figma `3:144`).
///
/// `bg-ink text-white`. Left: H2 "Let's find your flow." (Fraunces ~72px,
/// `leading-none`) + "Ready to move with intention?" (`text-white/60`) + the
/// `mailto:` email link (Geist Medium 24px, `text-terracotta`, underlined).
/// Right: a link row (Services / Approach / FAQ / Policies) + Instagram &
/// LinkedIn icons (24px). Then a full-width hairline, the copyright, and a
/// `chpilates.fun` wordmark (Fraunces Bold 18px).
///
/// A direct `get_design_context` pull on `3:144` supplied the exact layout and
/// copy (gaps `80/32/24/16px`, the two-column top frame, the hairline + bottom
/// row). Deliberate deviations from the raw node, per this task's spec:
/// - **© year is 2026**, not the design's stale 2024.
/// - the design's `whitespace-nowrap` on the left column is dropped so the
///   72px H1 can `clamp()` down and wrap on a phone instead of overflowing.
///
/// Link targets are honest — no fabricated routes. "Services" and "Approach"
/// both resolve to `#services` (the Approach/Philosophy section owns that
/// anchor; there is no separate `#approach`); FAQ / Policies are future pages
/// with no route, so they use `#` placeholders; Instagram → the real profile;
/// LinkedIn has no known URL, so `#` placeholder.
///
/// Responsive: the two columns stack under `md` (`flex-col md:flex-row`), the
/// link row wraps (`flex-wrap`), the bottom row stacks, and the H2 `clamp()`s
/// so nothing overflows a narrow viewport. `font-sans` is set on the root so
/// the footer renders in Geist even though it sits outside `<main>`. The
/// `mailto:` link also carries `break-words` so the address itself can never
/// force horizontal scroll at very narrow widths.
///
/// This section's heading is an `<h2>`, not an `<h1>` — Hero owns the page's
/// sole `<h1>` ("Chean Hui Toh"); a page must have exactly one.
#[allow(clippy::must_use_candidate)]
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="flex flex-col gap-16 bg-ink px-6 pb-20 pt-24 font-sans text-white md:gap-20 md:px-[120px] md:pb-[120px] md:pt-[160px]">
            <div class="flex w-full flex-col gap-12 md:flex-row md:items-end md:justify-between">
                <div class="flex w-full flex-col items-start gap-8 md:w-[600px]">
                    <h2 class="font-display text-[clamp(2.5rem,8vw,72px)] leading-none text-white">
                        "Let’s find your flow."
                    </h2>
                    <div class="flex flex-col items-start gap-2">
                        <p class="text-lg text-white/60">"Ready to move with intention?"</p>
                        <a
                            href="mailto:hello@chpilates.fun"
                            class="break-words font-sans text-2xl font-medium text-terracotta underline"
                        >
                            "hello@chpilates.fun"
                        </a>
                    </div>
                </div>

                <div class="flex flex-col items-start gap-6 md:items-end">
                    <nav class="flex flex-wrap gap-x-12 gap-y-3 text-[15px]">
                        <a href="#services" class="text-white/80">
                            "Services"
                        </a>
                        <a href="#services" class="text-white/80">
                            "Approach"
                        </a>
                        <a href="#" class="text-white/80">
                            "FAQ"
                        </a>
                        <a href="#" class="text-white/80">
                            "Policies"
                        </a>
                    </nav>
                    <div class="flex items-center gap-4">
                        <a
                            href=INSTAGRAM_URL
                            target="_blank"
                            rel="noopener noreferrer"
                            aria-label="Instagram"
                        >
                            <IconInstagram class="w-6 h-6".to_string() />
                        </a>
                        <a href="#" aria-label="LinkedIn">
                            <IconLinkedIn class="w-6 h-6".to_string() />
                        </a>
                    </div>
                </div>
            </div>

            <div class="flex w-full flex-col gap-8">
                <div class="h-px w-full bg-white/15"></div>
                <div class="flex w-full flex-col gap-4 md:flex-row md:items-center md:justify-between">
                    <p class="text-sm text-white/40">
                        "© 2026 Chean Hui Toh. Built with breath."
                    </p>
                    <p class="font-display text-[18px] font-bold text-white/40">"chpilates.fun"</p>
                </div>
            </div>
        </footer>
    }
}
