use leptos::prelude::*;

/// Hero — `docs/frontend.md` §2 (Figma `3:18`, 1440×900).
///
/// Full-bleed `/img/hero.jpg` with a `bg-black/5` overlay, a centered
/// Fraunces wordmark + `PILATES • PHYSIOTHERAPY • MOVEMENT` label row (each
/// word separated by an `8px` `bg-terracotta` dot), and a bottom-center
/// scroll cue. This is the sole owner of `id="top"` (the Nav wordmark link
/// targets it; `HomePage` no longer puts `id="top"` on `<main>`).
///
/// The Figma frame is a fixed 1440×900 desktop layout; two rules keep it
/// sane on a phone-sized viewport instead of imposing a giant fixed block or
/// clipping the label row:
/// - height is `min-h-[600px] h-[100svh] md:h-[900px]` — it fills the
///   viewport (via small-viewport-height units, which stay correct behind
///   mobile browser chrome) up to a sensible floor, then locks to the
///   design's `900px` from `md` up.
/// - the wordmark keeps the design's `clamp()` down to `3.5rem` so `112px`
///   scales with viewport width instead of overflowing a narrow screen.
/// - the label row wraps (`flex-wrap`) and starts at a smaller size/gap,
///   reaching the design's `18px` text / `16px` gap only from `md` up, so
///   "Pilates • Physiotherapy • Movement" never forces horizontal scroll.
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
