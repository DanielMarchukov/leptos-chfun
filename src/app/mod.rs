use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

mod components;
mod icons;
mod nav;
mod sections;

pub use components::{CardFill, Eyebrow, InfoCard};
pub use icons::{
    IconAmbulance, IconAward, IconBriefcase, IconGraduationCap, IconInfinity, IconInstagram,
    IconLinkedIn, IconMenu, IconWind, IconX,
};
pub use nav::Nav;
pub use sections::{About, Approach, Credentials, Hero};

/// Shared copy for `<title>`/`<meta name="description">`/Open Graph — keeps
/// the practitioner's name in the title (a CI check greps the served HTML
/// for it) and a single source of truth for the OG title/description pair.
const SITE_TITLE: &str = "Chean Hui Toh — Pilates, Physiotherapy & Movement";
const SITE_DESCRIPTION: &str = "Chean Hui Toh brings physiotherapy expertise to Pilates and \
     movement coaching — sessions built to ease pain, build strength, and restore fluid, \
     confident movement.";

#[must_use]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <Link
                    rel="preload"
                    href="/fonts/fraunces-variable.woff2"
                    as_="font"
                    type_="font/woff2"
                    crossorigin="anonymous"
                />
                <Link
                    rel="preload"
                    href="/fonts/geist-variable.woff2"
                    as_="font"
                    type_="font/woff2"
                    crossorigin="anonymous"
                />
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
        <Title text=SITE_TITLE/>
        <Meta name="description" content=SITE_DESCRIPTION/>
        <Meta property="og:title" content=SITE_TITLE/>
        <Meta property="og:description" content=SITE_DESCRIPTION/>
        <Meta property="og:type" content="website"/>
        <Router>
            <Routes fallback=|| "Not found.".into_view()>
                <Route path=StaticSegment("") view=HomePage/>
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Nav/>
        <main class="bg-cream text-ink font-sans">
            <Hero/>
            <About/>
            <Credentials/>
            <Approach/>
            // Placeholder — Tasks 7-8 insert the remaining section stack
            // here, in order: Gallery, Testimonial, Instagram ("Follow the
            // Flow"), Footer. `#gallery`/`#connect` land on the Gallery and
            // Instagram sections respectively; `#services` is already wired
            // up on `Approach`.
            <div class="px-6 py-24 text-center text-ink/60">
                <p>"More sections land here in Tasks 7-8."</p>
            </div>
        </main>
    }
}
