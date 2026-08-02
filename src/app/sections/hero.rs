use leptos::prelude::*;

/// Hero (Figma `3:18`) — full-bleed photo, Fraunces wordmark, terracotta-dot
/// label row, scroll cue; sole owner of `id="top"`. See README (Design notes).
#[allow(clippy::must_use_candidate)]
#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section
            id="top"
            class="relative flex min-h-[600px] h-[100svh] md:h-[900px] flex-col items-center justify-center overflow-hidden"
        >
            <img
                src="/img/hero.jpg"
                alt="Chean Hui Toh in a Pilates pose in a sunlit studio"
                class="absolute inset-0 size-full object-cover"
            />
            <div class="absolute inset-0 bg-black/5"></div>

            <div class="relative flex w-[800px] max-w-[90vw] flex-col items-center gap-6 text-center">
                <h1 class="font-display text-[clamp(3.5rem,9vw,112px)] leading-none text-ink">
                    "Chean Hui Toh"
                </h1>
                <div class="flex flex-wrap items-center justify-center gap-x-3 gap-y-2 text-sm uppercase text-ink md:gap-x-4 md:text-[18px]">
                    <span>"Pilates"</span>
                    <span class="size-2 shrink-0 rounded-[4px] bg-terracotta"></span>
                    <span>"Physiotherapy"</span>
                    <span class="size-2 shrink-0 rounded-[4px] bg-terracotta"></span>
                    <span>"Movement"</span>
                </div>
            </div>

            <div class="absolute bottom-16 flex flex-col items-center gap-3">
                <span class="text-sm text-ink/60">"chpilates.fun"</span>
                <span class="h-[60px] w-px bg-ink/60"></span>
            </div>
        </section>
    }
}
