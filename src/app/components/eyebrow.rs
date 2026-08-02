use leptos::prelude::*;

/// Small "dash + uppercase label" heading shown above section titles. See
/// README (Design notes: Radii & spacing).
#[allow(clippy::must_use_candidate)]
#[component]
pub fn Eyebrow(
    /// The label text, rendered uppercase.
    #[prop(into)]
    label: String,
    /// Full Tailwind text-color utility for the dash + label (must be a
    /// literal, e.g. `"text-sage"`, so Tailwind's scanner can see it).
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
