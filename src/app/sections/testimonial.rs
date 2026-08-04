use leptos::prelude::*;

/// Testimonial (Figma `3:123`) — sage-tint quote band + avatar attribution.
/// No `id`; placeholder content. See README (Design notes, Key decisions).
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
