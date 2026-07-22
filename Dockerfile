# syntax=docker/dockerfile:1.7

# Rust tracks the latest stable release, matching pr_open.yml's
# dtolnay/rust-toolchain@stable. DEBIAN_VERSION pins the base OS and is shared by
# both stages so the builder's glibc matches the runtime's.
ARG DEBIAN_VERSION=bookworm

# ----------------------------------------------------------------------
# Stage 1: build server binary + hydration wasm + site assets
# ----------------------------------------------------------------------
FROM rust:slim-${DEBIAN_VERSION} AS builder

# binaryen ships wasm-opt, used by cargo-leptos when optimizing the hydration
# bundle. ca-certificates lets cargo fetch from crates.io over HTTPS.
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        binaryen \
        ca-certificates \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

# cargo-leptos downloads the Tailwind v4 standalone CLI itself at build time.
# Pin the version for reproducible builds; bump deliberately and keep in sync
# with pr_open.yml.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo install --locked --version 0.3.7 cargo-leptos

WORKDIR /app
COPY . .

# GIT_SHA is captured at COMPILE time by option_env!("GIT_SHA") in
# src/server/http.rs, so it must be set before `cargo leptos build` runs.
# Defaults to "unknown" so a local `docker build` without --build-arg still
# produces a usable image.
ARG GIT_SHA=unknown
ENV GIT_SHA=${GIT_SHA}

# Cache mounts on registry/git/target make repeat builds fast. `target/` is
# not written into the image layer, so copy the build outputs to /out inside
# this same RUN before the cache mount detaches.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo leptos build --release \
    && mkdir -p /out \
    && cp target/release/leptos-chfun /out/server \
    && cp -r target/site /out/site

# ----------------------------------------------------------------------
# Stage 2: run the axum SSR server
# ----------------------------------------------------------------------
FROM debian:${DEBIAN_VERSION}-slim AS runtime

# wget powers the HEALTHCHECK below. Security headers, gzip, caching, and the
# dotfile guard that nginx used to provide now live in the axum middleware
# (src/server/http.rs), so the runtime here is just the server binary.
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        wget \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /out/server /app/server
COPY --from=builder /out/site   /app/site

ENV LEPTOS_OUTPUT_NAME=leptos-chfun \
    LEPTOS_SITE_ROOT=site \
    LEPTOS_SITE_PKG_DIR=pkg \
    LEPTOS_SITE_ADDR=0.0.0.0:80

# Serve on 80 like the previous nginx image, so the existing Dokploy/Traefik
# routing (with Cloudflare in front) stays unchanged.
EXPOSE 80

HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
    CMD wget --quiet --tries=1 --spider http://localhost/healthz || exit 1

CMD ["/app/server"]
