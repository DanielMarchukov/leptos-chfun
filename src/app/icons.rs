//! Lucide icon set used across the landing page (MIT-licensed, lucide.dev).
//!
//! Path data is copied verbatim from Lucide, not redrawn. `graduation-cap`,
//! `award`, `briefcase`, `ambulance`, `wind`, `infinity`, `menu`, `x` come
//! from the current `lucide-static` package (`viewBox="0 0 24 24"`). Lucide
//! later dropped its brand icons (trademark policy); `instagram` and
//! `linkedin` are pinned from `lucide-static@1.0.0`, the last release that
//! carried them — the path data matches the `IconInstagram` snippet in
//! `docs/frontend.md` §"Icons (Lucide)" exactly, confirming it's the same
//! artwork the design spec expects.
//!
//! Every icon is `stroke="currentColor"` + `fill="none"`, so color follows
//! the parent's `text-*` utility. Sizing is via a `class` prop (not a
//! numeric `size`) so callers can hand it any Tailwind size utility
//! (`w-7 h-7` in a card, `w-5 h-5` in the nav, …); it defaults to `w-6 h-6`
//! (Lucide's native 24px) when omitted.

use leptos::prelude::*;

/// `graduation-cap` — Credentials: "Qualifications".
#[allow(clippy::must_use_candidate)]
#[component]
pub fn IconGraduationCap(
    #[prop(into, default = "w-6 h-6".to_string())] class: String,
) -> impl IntoView {
    view! {
        <svg
            class=class
            aria-hidden="true"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M21.42 10.922a1 1 0 0 0-.019-1.838L12.83 5.18a2 2 0 0 0-1.66 0L2.6 9.08a1 1 0 0 0 0 1.832l8.57 3.908a2 2 0 0 0 1.66 0z" />
            <path d="M22 10v6" />
            <path d="M6 12.5V16a6 3 0 0 0 12 0v-3.5" />
        </svg>
    }
}

/// `award` — Credentials: "Training & Certifications".
#[allow(clippy::must_use_candidate)]
#[component]
pub fn IconAward(#[prop(into, default = "w-6 h-6".to_string())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            aria-hidden="true"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="m15.477 12.89 1.515 8.526a.5.5 0 0 1-.81.47l-3.58-2.687a1 1 0 0 0-1.197 0l-3.586 2.686a.5.5 0 0 1-.81-.469l1.514-8.526" />
            <circle cx="12" cy="8" r="6" />
        </svg>
    }
}

/// `briefcase` — Credentials: "Experience".
#[allow(clippy::must_use_candidate)]
#[component]
pub fn IconBriefcase(
    #[prop(into, default = "w-6 h-6".to_string())] class: String,
) -> impl IntoView {
    view! {
        <svg
            class=class
            aria-hidden="true"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M16 20V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" />
            <rect width="20" height="14" x="2" y="6" rx="2" />
        </svg>
    }
}

/// `ambulance` — Approach: "Physio-Informed".
#[allow(clippy::must_use_candidate)]
#[component]
pub fn IconAmbulance(
    #[prop(into, default = "w-6 h-6".to_string())] class: String,
) -> impl IntoView {
    view! {
        <svg
            class=class
            aria-hidden="true"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M10 10H6" />
            <path d="M14 18V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v11a1 1 0 0 0 1 1h2" />
            <path d="M19 18h2a1 1 0 0 0 1-1v-3.28a1 1 0 0 0-.684-.948l-1.923-.641a1 1 0 0 1-.578-.502l-1.539-3.076A1 1 0 0 0 16.382 8H14" />
            <path d="M8 8v4" />
            <path d="M9 18h6" />
            <circle cx="17" cy="18" r="2" />
            <circle cx="7" cy="18" r="2" />
        </svg>
    }
}

/// `wind` — Approach: "Breath as Anchor".
#[allow(clippy::must_use_candidate)]
#[component]
pub fn IconWind(#[prop(into, default = "w-6 h-6".to_string())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            aria-hidden="true"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M12.8 19.6A2 2 0 1 0 14 16H2" />
            <path d="M17.5 8a2.5 2.5 0 1 1 2 4H2" />
            <path d="M9.8 4.4A2 2 0 1 1 11 8H2" />
        </svg>
    }
}

/// `infinity` — Approach: "Fluid Resilience".
#[allow(clippy::must_use_candidate)]
#[component]
pub fn IconInfinity(#[prop(into, default = "w-6 h-6".to_string())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            aria-hidden="true"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M6 16c5 0 7-8 12-8a4 4 0 0 1 0 8c-5 0-7-8-12-8a4 4 0 1 0 0 8" />
        </svg>
    }
}

/// `instagram` — nav, footer, Instagram section handle pill / CTA.
#[allow(clippy::must_use_candidate)]
#[component]
pub fn IconInstagram(
    #[prop(into, default = "w-6 h-6".to_string())] class: String,
) -> impl IntoView {
    view! {
        <svg
            class=class
            aria-hidden="true"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <rect width="20" height="20" x="2" y="2" rx="5" ry="5" />
            <path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z" />
            <line x1="17.5" x2="17.51" y1="6.5" y2="6.5" />
        </svg>
    }
}

/// `linkedin` — footer.
#[allow(clippy::must_use_candidate)]
#[component]
pub fn IconLinkedIn(#[prop(into, default = "w-6 h-6".to_string())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            aria-hidden="true"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z" />
            <rect width="4" height="12" x="2" y="9" />
            <circle cx="4" cy="4" r="2" />
        </svg>
    }
}

/// `menu` — mobile nav toggle (closed state).
#[allow(clippy::must_use_candidate)]
#[component]
pub fn IconMenu(#[prop(into, default = "w-6 h-6".to_string())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            aria-hidden="true"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M4 5h16" />
            <path d="M4 12h16" />
            <path d="M4 19h16" />
        </svg>
    }
}

/// `x` — mobile nav toggle (open state).
#[allow(clippy::must_use_candidate)]
#[component]
pub fn IconX(#[prop(into, default = "w-6 h-6".to_string())] class: String) -> impl IntoView {
    view! {
        <svg
            class=class
            aria-hidden="true"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M18 6 6 18" />
            <path d="m6 6 12 12" />
        </svg>
    }
}
