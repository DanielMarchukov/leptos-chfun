use leptos::prelude::*;

use crate::app::{CardFill, Eyebrow, IconAmbulance, IconInfinity, IconWind, InfoCard};

/// Approach — "The Philosophy" (Figma `3:43`), `id="services"` — canonical
/// `InfoCard` fill palette reference (sage-tint / blush / sage-tint). See
/// README (Design notes).
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
