use leptos::prelude::*;

use crate::app::Eyebrow;

/// Gallery — "Movement Moments" (Figma `3:68`), `id="gallery"`. Curated static
/// asymmetric photo grid, not the IG feed. See README (Design notes).
#[allow(clippy::must_use_candidate)]
#[component]
pub fn Gallery() -> impl IntoView {
    view! {
        <section
            id="gallery"
            class="flex flex-col items-center gap-10 bg-cream px-6 py-16 md:gap-16 md:px-[120px] md:py-[160px]"
        >
            <div class="flex w-full flex-col items-start gap-6 md:flex-row md:items-end md:justify-between">
                <div class="flex flex-col items-start gap-4">
                    <Eyebrow label="Movement Moments" />
                    <h2 class="font-display text-[clamp(2rem,5vw,48px)] leading-[1.1] text-ink">
                        "Visualizing the flow."
                    </h2>
                </div>
                <p class="max-w-full text-base leading-[1.6] text-ink/60 md:max-w-[420px] md:text-center">
                    "A collection of snapshots from my studio and practice, capturing the intersection of effort and ease."
                </p>
            </div>

            <div class="flex w-full flex-col gap-6">
                <div class="grid w-full grid-cols-1 gap-6 md:grid-cols-[792fr_384fr]">
                    <div class="aspect-[4/3] w-full overflow-hidden rounded-[24px] md:aspect-auto md:h-[200px] lg:h-[400px]">
                        <img
                            src="/img/gallery-01.jpg"
                            alt="Close-up of a wooden Pilates reformer machine in the studio"
                            class="size-full object-cover"
                        />
                    </div>
                    <div class="aspect-[4/3] w-full overflow-hidden rounded-[24px] md:aspect-auto md:h-[200px] lg:h-[400px]">
                        <img
                            src="/img/gallery-02.jpg"
                            alt="Close-up of hands guiding a stretch during a session"
                            class="size-full object-cover"
                        />
                    </div>
                </div>

                <div class="grid w-full grid-cols-1 gap-6 md:grid-cols-[384fr_792fr]">
                    <div class="aspect-[4/3] w-full overflow-hidden rounded-[24px] md:aspect-auto md:h-[250px] lg:h-[500px]">
                        <img
                            src="/img/gallery-03.jpg"
                            alt="Sunlit studio hallway lined with Pilates reformers"
                            class="size-full object-cover"
                        />
                    </div>
                    <div class="aspect-[4/3] w-full overflow-hidden rounded-[24px] md:aspect-auto md:h-[250px] lg:h-[500px]">
                        <img
                            src="/img/gallery-04.jpg"
                            alt="Chean Hui guiding a client through a reformer exercise"
                            class="size-full object-cover"
                        />
                    </div>
                </div>

                <div class="grid w-full grid-cols-1 gap-6 md:grid-cols-2">
                    <div class="aspect-[4/3] w-full overflow-hidden rounded-[24px] md:aspect-auto md:h-[225px] lg:h-[450px]">
                        <img
                            src="/img/gallery-05.jpg"
                            alt="A water bottle and cork mat set up for a session"
                            class="size-full object-cover"
                        />
                    </div>
                    <div class="aspect-[4/3] w-full overflow-hidden rounded-[24px] md:aspect-auto md:h-[225px] lg:h-[450px]">
                        <img
                            src="/img/gallery-06.jpg"
                            alt="Flowing fabric evoking fluid, breath-led movement"
                            class="size-full object-cover"
                        />
                    </div>
                </div>
            </div>
        </section>
    }
}
