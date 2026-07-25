use leptos::prelude::*;

use crate::app::Eyebrow;

/// Gallery — "Movement Moments" — `docs/frontend.md` §6 (Figma `3:68`).
///
/// `bg-cream`. Header row: left column (`Eyebrow "Movement Moments"` + H2
/// `"Visualizing the flow."`) beside a supporting paragraph on the right;
/// stacks (text above paragraph) under `md`. Below it, an **asymmetric photo
/// grid** of the six curated `/img/gallery-0N.jpg` stills — this is static
/// content, never the live Instagram feed (`InstagramSection`, Task 8, owns
/// `get_feed()`). `id="gallery"` is the Nav "Gallery" anchor target.
///
/// A direct `get_design_context` pull on `3:68` confirmed the values past
/// what `frontend.md`'s prose captures:
/// - tile corner radius is `24px` (`rounded-[24px]`), not the card radius
///   (`40px`) used by `InfoCard`.
/// - the three rows are literally `792:384`, `384:792`, and `588:588` (px)
///   at heights `400`/`500`/`450`, gap `24px` both between and within rows —
///   implemented as three `md:grid-cols-[<w1>fr_<w2>fr]` rows so the ratio
///   (not the absolute px) holds at any content width, with a `24px` gap
///   (`gap-6`) throughout. Each row collapses to `grid-cols-1` (stacked,
///   full width) below `md`, and tiles use a `4/3` aspect ratio at that
///   width instead of the desktop fixed pixel heights (fixed heights sized
///   for a 1200px-wide desktop row look wrong forced onto a phone).
/// - the desktop heights are gated behind `lg:`, not `md:`: at `md` (~768px)
///   the same two-column layout kicks in but the row is only ~500px wide, so
///   the full `400/500/450px` heights would crop the narrow column into a
///   tall, awkward sliver. `md:h-[200/250/225px]` (roughly the same
///   width-scaled ratio) fills the tablet range, and `lg:h-[400/500/450px]`
///   restores the design's exact desktop heights from `1024px` up.
/// - **Conflict with this task's written spec:** node `3:68` renders the
///   "Movement Moments" eyebrow dash + label in **terracotta** (`#d4a38a`),
///   not sage — the same About-section-style deviation already flagged in
///   `about.rs`. Implemented per the written spec (`Eyebrow`'s sage
///   default) rather than silently matching the live design; flagged here
///   for the same design follow-up.
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
