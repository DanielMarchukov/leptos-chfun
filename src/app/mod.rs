use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

mod components;
mod icons;

pub use components::{CardFill, Eyebrow, InfoCard};
pub use icons::{
    IconAmbulance, IconAward, IconBriefcase, IconGraduationCap, IconInfinity, IconInstagram,
    IconLinkedIn, IconMenu, IconWind, IconX,
};

#[must_use]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

// `#[component]` generates a `pub fn App`; the `must_use_candidate` pedantic
// lint fires on it, but the return value is always consumed by the framework.
#[allow(clippy::must_use_candidate)]
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-chfun.css"/>
        <Title text="chpilates.fun"/>
        <Router>
            <main>
                <Routes fallback=|| "Not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // Design-system smoke test: Task 1's token utilities plus Task 2's
    // primitives (Eyebrow, InfoCard, a few Lucide icons) so the SSR HTML can
    // be curl-checked for the expected markup/classes. Real sections replace
    // this in later Phase 2 tasks.
    view! {
        <main class="min-h-screen bg-cream text-ink font-sans px-6 py-16">
            <h1 class="font-display text-5xl">
                "Chean Hui " <span class="text-terracotta">"Toh"</span>
            </h1>
            <p class="mt-4 text-lg text-ink/60">"Pilates · Physiotherapy · Movement"</p>

            <div class="mt-12 flex items-center gap-6 text-ink">
                <IconGraduationCap class="w-7 h-7".to_string() />
                <IconInstagram class="w-5 h-5".to_string() />
                <IconMenu />
            </div>

            <div class="mt-8">
                <Eyebrow label="Credentials & Training".to_string() />
            </div>

            <div class="mt-6 grid max-w-md grid-cols-1 gap-8">
                <InfoCard
                    icon=move || view! { <IconGraduationCap class="w-7 h-7".to_string() /> }
                    title="Qualifications".to_string()
                    body="Bachelor of Physiotherapy, Certified Pilates Instructor (STOTT)."
                        .to_string()
                    fill=CardFill::Blush
                />
            </div>
        </main>
    }
}
