# syntax=docker/dockerfile:1.7

# Rust tracks stable, matching pr_open.yml's toolchain. DEBIAN_VERSION pins
# the OS for both stages so glibc matches between builder and runtime.
ARG DEBIAN_VERSION=bookworm

# ----------------------------------------------------------------------
# Stage 1: build server binary + hydration wasm + site assets
# ----------------------------------------------------------------------
FROM rust:slim-${DEBIAN_VERSION} AS builder

# binaryen -> wasm-opt for cargo-leptos. libssl-dev/pkg-config/perl/make let
# openssl-sys build against system OpenSSL (slim base ships none).
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        binaryen \
        ca-certificates \
        pkg-config \
        libssl-dev \
        perl \
        make \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

# Also downloads the Tailwind v4 CLI at build time. Version pinned for
# reproducible builds; keep in sync with pr_open.yml.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo install --locked --version 0.3.7 cargo-leptos

WORKDIR /app
COPY . .

# Captured at compile time by option_env!("GIT_SHA") in src/server/http.rs;
# defaults to "unknown" for a plain local build.
ARG GIT_SHA=unknown
ENV GIT_SHA=${GIT_SHA}

# cargo-leptos fetches wasm-bindgen/tailwind from GitHub at build time, which can
# transiently time out — retry so a network blip can't fail CI (cached crates
# make retries cheap). target/ isn't in the image layer, so copy outputs to /out
# before the mount detaches.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    n=0; \
    until cargo leptos build --release; do \
        n=$((n + 1)); \
        [ "$n" -ge 5 ] && { echo "cargo leptos build failed after $n attempts"; exit 1; }; \
        echo "build attempt $n hit a transient failure; retrying in $((n * 10))s..."; \
        sleep $((n * 10)); \
    done \
    && mkdir -p /out \
    && cp target/release/leptos-chfun /out/server \
    && cp -r target/site /out/site

# ----------------------------------------------------------------------
# Stage 2: run the axum SSR server
# ----------------------------------------------------------------------
FROM debian:${DEBIAN_VERSION}-slim AS runtime

# wget powers HEALTHCHECK; libcap2-bin provides setcap so the non-root server
# can bind port 80. nginx's old duties now live in axum middleware (src/server/http.rs).
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        wget \
        libcap2-bin \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /out/server /app/server
COPY --from=builder /out/site   /app/site

# Non-root; setcap grants cap_net_bind_service so binding port 80 needs
# neither root nor a port change.
RUN useradd --system --no-create-home --user-group --uid 10001 app \
    && chown -R app:app /app \
    && setcap 'cap_net_bind_service=+ep' /app/server

ENV LEPTOS_OUTPUT_NAME=leptos-chfun \
    LEPTOS_SITE_ROOT=site \
    LEPTOS_SITE_PKG_DIR=pkg \
    LEPTOS_SITE_ADDR=0.0.0.0:80

# Port 80, matching the previous nginx image and existing Traefik routing.
EXPOSE 80

HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
    CMD wget --quiet --tries=1 --spider http://localhost/healthz || exit 1

USER app
CMD ["/app/server"]
