# leptos-chfun — chpilates.fun

Personal brand site for **Chean Hui Toh** (Pilates, physiotherapy & movement),
built as a server-side-rendered [Leptos](https://leptos.dev) app with
client-side WASM hydration. Ported from a Figma design (file
`7weEQyXBTpohVKQIhmhpPj`).

This README is the project's knowledge base: what's implemented, the key
decisions, and the per-section design notes. Code comments stay terse and point
here.

---

## Features

- **SSR + hydration** — Leptos 0.8 + `axum` + `cargo-leptos`; the server renders
  HTML and the client hydrates it (predominantly-WASM, no islands). Split
  lib+bin crate; hydrate entry in `src/lib.rs`.
- **Tailwind v4 design system** — `@theme` tokens in `style/tailwind.css`
  (`ink #2d2926`, `terracotta #d4a38a`, `sage #8fa382`, `blush #f5e9e2`,
  `sage-tint #e9ede6`, `cream #faf6f2`), self-hosted **Fraunces** (display) +
  **Geist** (sans) woff2, and **Lucide** icons.
- **Nine sections** — sticky **Nav** (mobile hamburger menu via a signal +
  `<Show>`), **Hero**, **About**, **Credentials**, **Approach**, **Gallery**,
  **Testimonial**, **Instagram**, **Footer**.
- **Shared primitives** — `Eyebrow` (dash + uppercase label), `InfoCard` (the
  canonical filled card, body via a `children` slot), and the Lucide icon set
  (`class`-prop sizing, `stroke="currentColor"`, `aria-hidden`).
- **Instagram feed contract** — `FeedItem` / `FeedKind` (`src/models.rs`, shared
  across ssr/hydrate) + a `get_feed` `#[server]` function. Ships a **Phase-4
  stub** (`Ok(vec![])`) so the section renders static fallback tiles today; the
  real cache-backed fetch drops in later with no UI change.
- **Mobile-first responsive** — every section works from ~320px phones to wide
  desktops with no horizontal overflow (clamped display type, stacking grids,
  nav → hamburger under `md`).
- **Image drop-in system** — every `<img>` is `object-cover` in a sized/aspect
  container, referencing a stable filename under `public/img/`. Swap in real
  photos by overwriting the file (keep the name + extension); `object-cover`
  crops any resolution. See [`public/img/README.md`](public/img/README.md).
- **Ops** — `tower-http` middleware reproducing the old nginx behaviour
  (security headers, gzip, cache tiers, dotfile guard) plus `/healthz` +
  `/version`; a non-root Docker image on port 80; a GHCR → Dokploy tag-deploy
  pipeline with a Dokploy-API rollout gate.

---

## Key decisions & conventions

- **Accent colors:** SAGE eyebrows + TERRACOTTA emphasis/accents, applied
  consistently site-wide. The live Figma About and Gallery nodes render
  *terracotta* eyebrows (and About renders "healing" in sage); these are treated
  as design slips and the consistent convention wins. Card-body `SemiBold`
  emphasis is `text-ink` (Geist 600), not terracotta.
- **Type sizes:** where a direct Figma node measurement differs from the generic
  type-scale table, the measured value wins (e.g. About H2 = 56px, body = 18px,
  not the table's 48/16).
- **Anchors & headings:** `#top` (Hero), `#about`, `#gallery`, `#services`
  (Approach — closest to "what she offers"; Credentials has no anchor),
  `#connect` (Instagram). Exactly one `<h1>` on the page (Hero's wordmark);
  every section header is `<h2>`, card titles `<h3>`.
- **Honest links:** no fabricated routes. "Services" and "Approach" both resolve
  to `#services` (there is no separate `#approach`); FAQ / Policies / LinkedIn
  are `#` placeholders for future pages/profiles; Instagram and the `mailto:`
  email are live.
- **Placeholders (owner-approved, to be replaced before go-live):** all imagery
  is stock extracted from Figma; the testimonial ("Elena R.") is placeholder
  copy; the footer FAQ/Policies/LinkedIn links are placeholders.
- **Deploy gate** polls the **Dokploy API** (`application.one` →
  `applicationStatus`), not the public `/version` through Cloudflare (Bot Fight
  Mode challenges the CI request). This is a deliberate **best-effort rollout
  signal**: it confirms Dokploy finished the rollout but does NOT correlate to
  this specific deploy/SHA. That's an accepted trade-off — the authoritative
  proof that the built image serves the right commit on `:80` is the PR
  **docker job**, which builds and runs this exact image and asserts `/version`.
  **Validated on live deploys (v0.3.0, v0.3.1):** the `applicationStatus` field
  and the `running`→`done` transition behave as documented and the gate exits
  clean. Per-SHA correlation is verified out-of-band by an origin-side
  `/version` SHA check (`curl --resolve`, bypassing Cloudflare).
- **CI split:** the PR verify job runs **fmt + clippy only** — the two clippy
  passes already compile both the `ssr` (native) and `hydrate` (wasm32) targets.
  The `cargo leptos build --release` + site assembly + a running-container smoke
  test live in the **Docker job**. `cargo leptos build` (release *and* debug)
  can't spawn its downloaded helper tools on the bare `ubuntu-latest` runner
  (ENOENT at cargo-leptos `src/ext/sync.rs:83`); it works only in the
  debian-bookworm build container.
- **Platform constraints:** the container binds **port 80** (Traefik routes to
  `:80`) as a **non-root** user; **latest-stable Rust** (not pinned);
  `cargo-leptos` pinned to `0.3.7`; Cloudflare Bot Fight Mode stays on; the
  origin IP is a repo secret. Never use `--all-features` on the wasm target (it
  pulls Tokio/mio, which doesn't compile for wasm).

---

## Design / section notes

Per-section detail pulled from the Figma design (`get_design_context`), beyond
what a token/type-scale table captures. Figma node ids in parentheses.

- **Nav** (`3:8`) — sticky, translucent cream + backdrop-blur, `px-6
  md:px-[120px]`. Wordmark `Chean Hui Toh / Pilates` → `#top`. Desktop links +
  Instagram icon show at `md`; below `md` a hamburger toggles a `<Show>` panel,
  swapping `IconMenu`↔`IconX`, with a reactive `aria-label`.
- **Hero** (`3:18`) — full-bleed `/img/hero.jpg` + `bg-black/5`, centered
  Fraunces wordmark, `PILATES • PHYSIOTHERAPY • MOVEMENT` label row (terracotta
  dots), scroll cue. Sole owner of `#top`. Responsive: height
  `min-h-[600px] h-[100svh] md:h-[900px]`; wordmark keeps the design's `clamp()`
  down to `3.5rem`; the label row wraps and scales so it never overflows.
- **About** (`3:30`) — two columns (stack under `md`). Portrait uses
  `rounded-tl/tr-[250px] rounded-bl/br-[20px]` on a `500×650` box — since
  `250+250 == 500`, the top stays a true arch at any width; the box is made
  fluid (`w-full max-w-[500px]` + `500/650` aspect ratio). Eyebrow "About Me"
  (sage), H2 56px with "healing" in a terracotta `<em class="not-italic">`, two
  body paragraphs at 18px.
- **Credentials** (`6:24`) — cream, sage eyebrow "Credentials & Training" + H2,
  three `InfoCard`s (fills `blush / sage-tint / blush`). Header-to-row gap 64px.
  Each card body is multiple `<p>` in the `InfoCard` `children` slot.
- **Approach** (`3:43`) — the **canonical** `InfoCard` palette reference. Cream,
  sage eyebrow "The Philosophy" + H2 "Integrating Science & Spirit" (header
  capped 700px), three cards (fills `sage-tint / blush / sage-tint`). Header-to-
  row gap 80px. Owns `#services`.
- **Gallery** (`3:68`) — cream, header row (eyebrow "Movement Moments" + H2
  "Visualizing the flow." + a supporting paragraph) then an **asymmetric** grid
  of six curated `/img/gallery-0N.jpg` (static content, *not* the IG feed).
  Rows are `792:384`, `384:792`, `588:588` (px), implemented as
  `md:grid-cols-[<w>fr_<w>fr]` so the ratio holds at any width; tiles
  `rounded-[24px] object-cover`, gap 24px. Below `md` each row stacks to one
  column with a `4/3` aspect; desktop heights (`400/500/450`) are gated at `lg:`
  (a scaled `md:h-[200/250/225]` fills the tablet range, since the full heights
  crop the narrow `md` column awkwardly). Owns `#gallery`.
- **Testimonial** (`3:123`) — sage-tint band, Fraunces `clamp(…,40px)` quote
  (max 800px), `size-14 rounded-full object-cover` avatar + attribution.
  Placeholder content.
- **Instagram** (`3:131`) — the only `get_feed()`-driven surface. Cream, H2
  "Follow the Flow" + a white handle pill (links to the profile), then the grid,
  then a dark "Follow on Instagram" CTA. Binds `get_feed` via a `Resource`
  (`Vec<FeedItem>` serializes SSR→hydrate, no client refetch); `<Suspense>` →
  `<Show>` swaps the real `<For>` grid (non-empty) for the static fallback
  (empty). The stub returns `[]` so the fallback always renders; the `<For>`
  branch + `FeedKind::Reel` play overlay are wired for Phase 4.
  `grid-cols-2 md:grid-cols-3`, `aspect-square`, `rounded-[24px]`. "Follow the
  Flow" is rendered Fraunces 400 for weight parity with the other section H2s
  (the design tags it SemiBold). Owns `#connect`.
- **Footer** (`3:144`) — `bg-ink text-white`. Left: H2 "Let's find your flow."
  (Fraunces ~72px, clamped), tagline, `mailto:` email (terracotta, underlined,
  `break-words`). Right: link row + Instagram/LinkedIn icons. Hairline, `© 2026`
  (design's 2024 is stale), `chpilates.fun` wordmark. Two columns stack under
  `md`; `font-sans` is set on the root since the footer sits outside `<main>`.

---

## Architecture

- `src/lib.rs` — crate root: `app` + `models` (shared) modules, ssr-only
  `server` module, and the `#[wasm_bindgen] hydrate()` entry.
- `src/main.rs` — the `axum` server (ssr only): `leptos_routes` (also registers
  `#[server]` functions) + status routes + `tower-http` middleware.
- `src/app/` — UI. `mod.rs` (shell `<head>`/meta, `App`, `HomePage`), `nav.rs`,
  `icons.rs`, `components/` (`eyebrow.rs`, `info_card.rs`), `sections/` (one file
  per section).
- `src/models.rs` — shared isomorphic types (`FeedItem`, `FeedKind`).
- `src/server/http.rs` — status routes (`/healthz`, `/version`) and the
  nginx-parity middleware (security headers, gzip, cache tiers, dotfile guard).
- `style/tailwind.css` — Tailwind v4 input: `@import "tailwindcss"`, `@theme`
  tokens, `@font-face` for the self-hosted fonts.
- `public/` — static assets served at the site root (`img/`, `fonts/`).
- `Dockerfile` — two-stage: `cargo-leptos` build → `debian:bookworm-slim`
  runtime running the server as a non-root user (uid 10001) on port 80
  (`setcap cap_net_bind_service`). `GIT_SHA` is compiled in via `option_env!`
  and served at `/version`.
- `.github/workflows/` — `pr_open.yml` (fmt + clippy + Docker build/run smoke
  test) and `build-and-deploy.yml` (tag → build → GHCR → Dokploy deploy +
  rollout gate + Cloudflare purge).

---

## Local development

### Prerequisites

- Rust (stable) with the `wasm32-unknown-unknown` target:
  `rustup target add wasm32-unknown-unknown`
- [`cargo-leptos`](https://github.com/leptos-rs/cargo-leptos), pinned to the
  version the build uses: `cargo install --locked --version 0.3.7 cargo-leptos`
- **Bash** and **curl** (used by `scripts/smoke-test.sh`)

### Run locally

```sh
cargo leptos watch
```

Open `http://localhost:3000` — the page hot-reloads on changes.

Port 3000 may already be taken locally (e.g. by a local Dokploy instance).
If so, override the site address:

```sh
LEPTOS_SITE_ADDR=127.0.0.1:3100 cargo leptos watch
```

and open `http://localhost:3100` instead.

### Run the smoke test

```sh
./scripts/smoke-test.sh
```

Builds the app, boots it on a local port, and asserts that `/` renders,
`/healthz` and `/version` respond, and the `/pkg/*.js` / `/pkg/*.wasm`
hydration assets serve. It defaults to port 3100; override with
`PORT=3200 ./scripts/smoke-test.sh` if that's also taken.

### Run the container locally

```sh
docker build -t leptos-chfun .
docker run --rm -p 8080:80 leptos-chfun
```

The container serves on port 80 (as in production); map any free host port to
it. Open `http://localhost:8080`.

### Verify like CI

```sh
cargo fmt --all -- --check
cargo clippy --features ssr -- -D warnings
cargo clippy --lib --target wasm32-unknown-unknown --features hydrate -- -D warnings
```

Never pass `--all-features` on the wasm target — it enables `ssr`, which pulls
Tokio's mio, which doesn't compile for wasm.
