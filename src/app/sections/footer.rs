use leptos::prelude::*;

use crate::app::{IconInstagram, IconLinkedIn, INSTAGRAM_URL};

/// Footer (Figma `3:144`) — dark footer, email CTA, link row, social icons,
/// copyright. Heading is `<h2>`; Hero owns the page's sole `<h1>`. See README
/// (Design notes, Key decisions).
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
