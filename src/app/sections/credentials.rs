use leptos::prelude::*;

use crate::app::{CardFill, Eyebrow, IconAward, IconBriefcase, IconGraduationCap, InfoCard};

/// Credentials — `docs/frontend.md` §4 (Figma `6:24`).
///
/// `bg-cream`, centered header (`Eyebrow "Credentials & Training"` in sage +
/// an H2), then a 3-card row of the canonical `InfoCard` (§5/Approach is the
/// palette reference; this section uses the identical treatment). Fill
/// sequence: `blush` / `sage-tint` / `blush`. No `id` — this section isn't a
/// Nav anchor target (`#services` lands on `Approach`).
///
/// A direct `get_design_context` pull on `6:24` supplied the full card copy
/// (`frontend.md`'s prose truncates it with "…") plus two details past the
/// written spec:
/// - the header-to-card-row gap is `64px` here, vs Approach's `80px` — kept
///   as literal per-section values rather than forcing them equal.
/// - the "Qualifications" and "Training & Certifications" cards each render
///   **two** body paragraphs (the "Experience" card too); `InfoCard`'s
///   `children` slot takes multiple `<p>` siblings for this, spaced by the
///   same `gap-4` (16px) as the title.
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
