use leptos::prelude::*;

/// The card's soft background tint.
///
/// `docs/frontend.md` §5 (Approach) is the canonical definition of this
/// card and calls out exactly two fills, alternated per card on the
/// `cream` page background — kept as an enum (not a stringly-typed class
/// prop) so a call site can't typo a fill, while each variant still maps to
/// one literal Tailwind utility that the build-time class scanner can see
/// below.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardFill {
    /// `--color-blush` (`#f5e9e2`).
    Blush,
    /// `--color-sage-tint` (`#e9ede6`).
    SageTint,
}

impl CardFill {
    const fn class(self) -> &'static str {
        match self {
            Self::Blush => "bg-blush",
            Self::SageTint => "bg-sage-tint",
        }
    }
}

/// The canonical filled info card — `docs/frontend.md` §4 (Credentials) and
/// §5 (Approach) define the *identical* card; §5 is the palette/design
/// reference. `rounded-[40px]` card, `p-12` (48px) padding, `gap-8` (32px)
/// between the icon chip and the text group, a `64px` white icon chip
/// (`rounded-[32px]`, 28px icon), a `28px` Fraunces title, `gap-4` (16px)
/// between the title and each body paragraph.
///
/// `icon` takes a `ViewFn` so a caller writes e.g.
/// `icon=move || view! { <IconGraduationCap class="w-7 h-7"/> }` — this
/// keeps `InfoCard` icon-agnostic (any `IntoView`, not just the `icons.rs`
/// set) while staying cheap to clone/re-run under SSR.
///
/// `children` is the card body, rendered in the same `gap-4` column as the
/// title — one or more `<p class="text-base leading-[1.6] text-ink/60">`
/// elements (a Credentials card renders two short paragraphs; an Approach
/// card renders one). That title+body column itself carries `font-sans` so
/// the body renders in Geist even if a future caller nests `InfoCard`
/// somewhere outside `<main>`'s ambient `font-sans` (as `Footer` already
/// does for its own content) — a single spot covering every card body
/// regardless of nesting, rather than relying on inherited context. The
/// title's own `font-display` class still wins on the `<h3>` itself (a
/// direct class on an element always overrides an inherited value from an
/// ancestor), so this doesn't change the title's font. This replaced an
/// earlier `body: String` prop: the
/// design's Credentials/Approach copy uses inline `SemiBold` emphasis spans
/// (`docs/frontend.md` §4 / the type-scale table's "Body" row), which a
/// `String` prop can't express — Leptos renders a `String` as escaped text,
/// not markup. A caller writes the emphasis directly, e.g.
/// `<p class="text-base leading-[1.6] text-ink/60">"I hold a "
/// <strong class="font-semibold text-ink">"Bachelor of Physiotherapy"</strong>
/// "…"</p>` — per the type scale, body emphasis is Geist-600 `text-ink`, not
/// terracotta (terracotta is reserved for in-heading accents, sage for
/// eyebrows).
// `icon: ViewFn` is only ever read via `.run(&self)` here, but Leptos props
// are owned by convention (the framework's own `<Show>`/`<Suspense>` accept
// `ViewFn`/`Children` the same way) — clippy can't see that convention.
#[allow(clippy::must_use_candidate, clippy::needless_pass_by_value)]
#[component]
pub fn InfoCard(
    /// Icon rendered inside the 64px white chip.
    #[prop(into)]
    icon: ViewFn,
    /// Card title (Fraunces 28px).
    #[prop(into)]
    title: String,
    /// Background tint — alternate `Blush`/`SageTint` per card.
    fill: CardFill,
    /// Card body — one or more `<p>` elements; see the doc comment above.
    children: Children,
) -> impl IntoView {
    view! {
        <article class=format!(
            "flex flex-1 flex-col gap-8 rounded-[40px] p-12 {}",
            fill.class(),
        )>
            <div class="flex h-16 w-16 items-center justify-center rounded-[32px] bg-white text-ink">
                {icon.run()}
            </div>
            <div class="flex flex-col gap-4 font-sans">
                <h3 class="font-display text-[28px] text-ink">{title}</h3>
                {children()}
            </div>
        </article>
    }
}
