use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
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
    // Design-system smoke test: exercises every token utility set up in Task 1
    // (bg-cream / text-ink / font-sans / font-display / text-terracotta /
    // bg-blush) so the compiled Tailwind v4 build and self-hosted fonts can be
    // verified visually. Real sections replace this in later Phase 2 tasks.
    view! {
        <main class="min-h-screen bg-cream text-ink font-sans px-6 py-16">
            <h1 class="font-display text-5xl">
                "Chean Hui " <span class="text-terracotta">"Toh"</span>
            </h1>
            <p class="mt-4 text-lg text-ink/60">
                "Pilates · Physiotherapy · Movement"
            </p>
            <div class="mt-8 inline-block rounded-[40px] bg-blush px-8 py-6">
                <p class="font-display text-2xl">"Design-system foundation"</p>
                <p class="mt-2 text-sm text-ink/60">
                    "Tailwind v4 tokens + self-hosted Fraunces & Geist"
                </p>
            </div>
        </main>
    }
}
