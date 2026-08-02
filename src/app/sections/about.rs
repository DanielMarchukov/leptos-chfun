use leptos::prelude::*;

use crate::app::Eyebrow;

/// About (Figma `3:30`), `id="about"` — arched portrait beside eyebrow/H2/body
/// copy, stacks under `md`. See README (Design notes, Key decisions).
#[allow(clippy::must_use_candidate)]
#[component]
pub fn About() -> impl IntoView {
    view! {
        <section
            id="about"
            class="flex flex-col items-center gap-10 px-6 py-16 md:flex-row md:gap-[100px] md:px-[120px] md:py-[160px]"
        >
            <div class="aspect-[500/650] w-full max-w-[500px] shrink-0 overflow-hidden rounded-tl-[250px] rounded-tr-[250px] rounded-bl-[20px] rounded-br-[20px]">
                <img
                    src="/img/about-portrait.jpg"
                    alt="Chean Hui Toh smiling in a sunlit Pilates studio"
                    class="size-full object-cover"
                />
            </div>

            <div class="flex w-full flex-1 flex-col items-start gap-10">
                <Eyebrow label="About Me" />
                <h2 class="font-display text-[clamp(2.25rem,5vw,56px)] leading-[1.1] text-ink">
                    "A bridge between " <em class="text-terracotta not-italic">"healing"</em>
                    " and the joy of movement."
                </h2>
                <div class="flex w-full flex-col gap-6 text-lg leading-[1.6] text-ink/60">
                    <p>
                        "I’m Chean Hui, a physiotherapist turned Pilates instructor. I believe in the power of mindful movement and conscious breathing to transform how we feel in our bodies."
                    </p>
                    <p>
                        "My sessions blend clinical knowledge with the joy of flowing movement — because healing should feel good. Whether you’re recovering from an injury or looking to deepen your practice, I’m here to guide you toward a more resilient, balanced self."
                    </p>
                </div>
            </div>
        </section>
    }
}
