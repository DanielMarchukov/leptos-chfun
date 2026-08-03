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
pub use sections::{
    About, Approach, Credentials, Footer, Gallery, Hero, InstagramSection, Testimonial,
};

/// Shared `<title>`/description/OG copy — keeps the practitioner's name in
/// the title (a CI check greps the served HTML for it).
const SITE_TITLE: &str = "Chean Hui Toh — Pilates, Physiotherapy & Movement";
const SITE_DESCRIPTION: &str = "Chean Hui Toh brings physiotherapy expertise to Pilates and \
     movement coaching — sessions built to ease pain, build strength, and restore fluid, \
     confident movement.";

/// Instagram profile — one source of truth for the nav, footer, and section.
pub(crate) const INSTAGRAM_HANDLE: &str = "ch.pilatesfun";
pub(crate) const INSTAGRAM_URL: &str = "https://instagram.com/ch.pilatesfun";

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

// `#[component]` generates `pub fn App`; must_use_candidate fires but the
// framework always consumes the return value.
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
            <Gallery/>
            <Testimonial/>
            <InstagramSection/>
        </main>
        <Footer/>
    }
}
