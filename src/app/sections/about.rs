use leptos::prelude::*;

use crate::app::Eyebrow;

/// About — `docs/frontend.md` §3 (Figma `3:30`, h=970).
///
/// Two columns on desktop: an arched portrait (`/img/about-portrait.jpg`)
/// beside an `Eyebrow "About Me"` + H2 + two body paragraphs. Stacks
/// (portrait above text) under `md`.
///
/// A direct `get_design_context` pull on `3:30` refined a few values past
/// what `frontend.md`'s prose/type-scale table capture:
/// - the portrait's radius is `rounded-tl-[250px] rounded-tr-[250px]
///   rounded-bl-[20px] rounded-br-[20px]` on a `500×650` box — CSS scales
///   down overlapping corner radii proportionally when they exceed the box
///   size, and `250 + 250 == 500` (the design width) means the top stays a
///   true semicircle arch at *any* width, not just the design's `500px`, so
///   these literal px radii are already responsive-safe.
/// - the H2 measures `56px` in this node (the type-scale table's generic
///   "Section H2 → 48px" row was sampled from other sections, not About)
///   and the body copy measures `18px`, not the general `16px` row.
/// - the portrait width is made fluid (`w-full max-w-[500px]`) with a
///   `500/650` `aspect-ratio` standing in for the design's fixed `650px`
///   height, so it never overflows a narrow viewport.
///
/// One confirmed conflict between the live design and the written spec,
/// left as spec'd rather than silently "fixed": node `3:30` renders the
/// "About Me" eyebrow in terracotta (`#d4a38a`) and "healing" in sage
/// (`#8fa382`) — the reverse of `frontend.md`'s color table (sage for
/// eyebrows, terracotta for in-heading emphasis) and this task's explicit
/// instructions. Implemented per the written spec (`Eyebrow`'s sage default,
/// terracotta "healing"); flagged for a design follow-up.
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
