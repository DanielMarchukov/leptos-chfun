use leptos::prelude::*;

/// The card's soft background tint — an enum (not a stringly-typed class
/// prop) so call sites can't typo a fill. See README (Design notes).
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

/// The canonical filled info card (Credentials + Approach) — icon chip,
/// Fraunces title, `children` body slot for one or more `<p>`s. See README
/// (Design notes).
// `icon: ViewFn` is owned by convention, like the framework's own `<Show>`/
// `<Suspense>` props; clippy's `needless_pass_by_value` can't see that.
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
