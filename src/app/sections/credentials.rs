use leptos::prelude::*;

use crate::app::{CardFill, Eyebrow, IconAward, IconBriefcase, IconGraduationCap, InfoCard};

/// Credentials (Figma `6:24`) — sage eyebrow + H2, three `InfoCard`s (blush /
/// sage-tint / blush). No `id`; not a Nav anchor. See README (Design notes).
#[allow(clippy::must_use_candidate)]
#[component]
pub fn Credentials() -> impl IntoView {
    view! {
        <section class="flex flex-col items-center gap-10 bg-cream px-6 py-16 md:gap-16 md:px-[120px] md:py-[160px]">
            <div class="flex w-full flex-col items-center gap-5 text-center">
                <Eyebrow label="Credentials & Training" />
                <h2 class="font-display text-[clamp(2rem,5vw,48px)] leading-[1.1] text-ink">
                    "A foundation of education and expertise."
                </h2>
            </div>

            <div class="grid w-full grid-cols-1 gap-8 md:grid-cols-3">
                <InfoCard
                    icon=move || view! { <IconGraduationCap class="w-7 h-7".to_string() /> }
                    title="Qualifications"
                    fill=CardFill::Blush
                >
                    <p class="text-base leading-[1.6] text-ink/60">
                        "I hold a " <strong class="font-semibold text-ink">"Bachelor of Physiotherapy"</strong>
                        " and am a " <strong class="font-semibold text-ink">"Certified Pilates Instructor"</strong>
                        " (STOTT or equivalent)."
                    </p>
                    <p class="text-base leading-[1.6] text-ink/60">
                        "I also completed a " <strong class="font-semibold text-ink">"Diploma in Clinical Pilates"</strong>
                        " to bridge clinical assessment with movement practice."
                    </p>
                </InfoCard>

                <InfoCard
                    icon=move || view! { <IconAward class="w-7 h-7".to_string() /> }
                    title="Training & Certifications"
                    fill=CardFill::SageTint
                >
                    <p class="text-base leading-[1.6] text-ink/60">
                        "Additional certifications include " <strong class="font-semibold text-ink">"Pre & Postnatal Pilates"</strong>
                        ", " <strong class="font-semibold text-ink">"Reformer Pilates Specialist"</strong>
                        ", and " <strong class="font-semibold text-ink">"Myofascial Release Techniques"</strong> "."
                    </p>
                    <p class="text-base leading-[1.6] text-ink/60">
                        "I continue to update my skills through ongoing workshops and professional development."
                    </p>
                </InfoCard>

                <InfoCard
                    icon=move || view! { <IconBriefcase class="w-7 h-7".to_string() /> }
                    title="Experience"
                    fill=CardFill::Blush
                >
                    <p class="text-base leading-[1.6] text-ink/60">
                        "With " <strong class="font-semibold text-ink">"8+ years in clinical physiotherapy"</strong>
                        " and " <strong class="font-semibold text-ink">"5+ years teaching Pilates"</strong>
                        ", I’ve worked with a range of clients."
                    </p>
                    <p class="text-base leading-[1.6] text-ink/60">
                        "From athletes and post-rehab clients to beginners, my goal is to make movement safe, accessible, and enjoyable."
                    </p>
                </InfoCard>
            </div>
        </section>
    }
}
