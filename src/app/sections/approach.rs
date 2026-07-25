use leptos::prelude::*;

use crate::app::{CardFill, Eyebrow, IconAmbulance, IconInfinity, IconWind, InfoCard};

/// Approach — "The Philosophy" — `docs/frontend.md` §5 (Figma `3:43`), the
/// **canonical** reference for the `InfoCard` fill palette/design.
///
/// `bg-cream`, centered header (`Eyebrow "The Philosophy"` in sage + an H2
/// capped at `700px`), then a 3-card row of `InfoCard`. Fill sequence:
/// `sage-tint` / `blush` / `sage-tint`. `id="services"` lives here (not on
/// Credentials) — it's the Nav "Services" anchor target, since this section
/// is closest to describing her method/practice.
///
/// A direct `get_design_context` pull on `3:43` supplied the full card copy
/// (`frontend.md`'s prose truncates it) and two details past the written
/// spec: the header-to-card-row gap is `80px` here (Credentials uses `64px`,
/// kept as a distinct per-section literal), and the header column is capped
/// at `700px` wide (Credentials' header has no such cap). Unlike Credentials,
/// each card here renders a single body paragraph with no `SemiBold`
/// emphasis spans.
#[allow(clippy::must_use_candidate)]
#[component]
pub fn Approach() -> impl IntoView {
    view! {
        <section
            id="services"
            class="flex flex-col items-center gap-10 bg-cream px-6 py-16 md:gap-20 md:px-[120px] md:py-[160px]"
        >
            <div class="flex w-full max-w-[700px] flex-col items-center gap-5 text-center">
                <Eyebrow label="The Philosophy" />
                <h2 class="font-display text-[clamp(2rem,5vw,48px)] leading-[1.1] text-ink">
                    "Integrating Science & Spirit"
                </h2>
            </div>

            <div class="grid w-full grid-cols-1 gap-8 md:grid-cols-3">
                <InfoCard
                    icon=move || view! { <IconAmbulance class="w-7 h-7".to_string() /> }
                    title="Physio-Informed"
                    fill=CardFill::SageTint
                >
                    <p class="text-base leading-[1.6] text-ink/60">
                        "Clinical expertise meets somatic awareness to ensure your movement is safe, effective, and tailored to your unique anatomy."
                    </p>
                </InfoCard>

                <InfoCard
                    icon=move || view! { <IconWind class="w-7 h-7".to_string() /> }
                    title="Breath as Anchor"
                    fill=CardFill::Blush
                >
                    <p class="text-base leading-[1.6] text-ink/60">
                        "Conscious breathing isn’t just a detail; it’s the engine of the movement, regulating the nervous system and deepening core connection."
                    </p>
                </InfoCard>

                <InfoCard
                    icon=move || view! { <IconInfinity class="w-7 h-7".to_string() /> }
                    title="Fluid Resilience"
                    fill=CardFill::SageTint
                >
                    <p class="text-base leading-[1.6] text-ink/60">
                        "We build strength that doesn’t feel rigid. Movement that is adaptable, free-flowing, and ready for the demands of real life."
                    </p>
                </InfoCard>
            </div>
        </section>
    }
}
