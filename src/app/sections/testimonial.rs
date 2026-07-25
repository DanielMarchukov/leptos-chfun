use leptos::prelude::*;

/// Testimonial — `docs/frontend.md` §7 (Figma `3:123`).
///
/// `bg-sage-tint` band, centered: a Fraunces 40px quote capped at `800px`
/// wide, then a `56px` round avatar (`/img/avatar-elena.jpg`) beside
/// "Elena R." (Geist SemiBold 16px) / "Client since 2021" (Geist 14px
/// `text-ink/60`). No `id` — not a Nav anchor target. Placeholder content
/// until a real testimonial exists.
///
/// A direct `get_design_context` pull on `3:123` supplied the exact quote
/// text and attribution (`frontend.md`'s prose only describes the layout),
/// plus the avatar radius: the design's `56px` frame is `rounded-[28px]`,
/// i.e. fully round at that size — implemented as `size-14 rounded-full`
/// per the image drop-in convention (round-crops any square-ish source),
/// which is equivalent at `56px` and stays round if the slot size ever
/// changes.
#[allow(clippy::must_use_candidate)]
#[component]
pub fn Testimonial() -> impl IntoView {
    view! {
        <section class="flex flex-col items-center gap-10 bg-sage-tint px-6 py-16 md:gap-20 md:px-[120px] md:py-[160px]">
            <p class="w-full max-w-[800px] text-center font-display text-[clamp(1.5rem,5vw,40px)] leading-[1.3] text-ink">
                "\"Chean Hui is a rare find. Her clinical eye as a physio completely changed how I approach my Pilates practice. I feel stronger and safer than ever before.\""
            </p>

            <div class="flex items-center gap-4">
                <img
                    src="/img/avatar-elena.jpg"
                    alt="Elena R."
                    class="size-14 rounded-full object-cover"
                />
                <div class="flex flex-col gap-0.5">
                    <span class="text-base font-semibold text-ink">"Elena R."</span>
                    <span class="text-sm text-ink/60">"Client since 2021"</span>
                </div>
            </div>
        </section>
    }
}
