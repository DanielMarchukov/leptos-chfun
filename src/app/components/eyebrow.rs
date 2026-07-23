use leptos::prelude::*;

/// Small "dash + uppercase label" heading placed above section titles
/// (`docs/frontend.md` §"Radii & spacing": `40px × 1px` dash, `12px` gap,
/// label).
///
/// `color_class` must be a *complete* Tailwind text-color utility (e.g. the
/// default `"text-sage"`) rather than a bare color name: the dash reuses it
/// via `bg-current` instead of duplicating a second `bg-*` class, and
/// Tailwind's build-time class scanner only ever sees whole literal utility
/// strings (in this file, or at call sites that pass a literal) — never a
/// runtime-assembled fragment like `format!("bg-{color}")`, which the
/// scanner can't see and so would be purged from the compiled CSS.
#[allow(clippy::must_use_candidate)]
#[component]
pub fn Eyebrow(
    /// The label text, rendered uppercase.
    #[prop(into)]
    label: String,
    /// Full Tailwind text-color utility for the dash + label. Every section
    /// in the design uses sage (`--color-sage`), hence the default.
    #[prop(into, default = "text-sage".to_string())]
    color_class: String,
) -> impl IntoView {
    let mut wrapper_class = color_class;
    wrapper_class.insert_str(0, "flex items-center gap-3 ");
    view! {
        <div class=wrapper_class>
            <span class="h-px w-10 bg-current"></span>
            <span class="font-sans text-sm font-semibold tracking-wide uppercase">{label}</span>
        </div>
    }
}
