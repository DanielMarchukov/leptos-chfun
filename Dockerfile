# syntax=docker/dockerfile:1.7

# Pinned versions — bump intentionally.
# Match TRUNK_VERSION here with .github/workflows/pr_open.yml so CI and
# production build the same artifact.
ARG RUST_VERSION=1.95
ARG TRUNK_VERSION=0.21.14
ARG NGINX_VERSION=1.31

# ----------------------------------------------------------------------
# Stage 1: build the Wasm bundle with trunk
# ----------------------------------------------------------------------
FROM rust:${RUST_VERSION}-slim AS builder

# binaryen ships wasm-opt, which trunk invokes when index.html sets
# data-wasm-opt="z". Without it trunk emits a warning and skips opt.
# ca-certificates lets cargo fetch from crates.io over HTTPS.
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        binaryen \
        ca-certificates \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

ARG TRUNK_VERSION
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo install --locked --version ${TRUNK_VERSION} trunk

WORKDIR /app
COPY . .

# Cache mounts on registry/git/target make repeat builds ~30 s once warm.
# `dist/` is written to the image layer (not the cache) so the runtime
# stage can COPY --from it.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    trunk build --release

# ----------------------------------------------------------------------
# Stage 2: serve dist/ with nginx
# ----------------------------------------------------------------------
FROM nginx:${NGINX_VERSION}-alpine AS runtime

# Replace the default vhost with our SPA-aware config.
COPY nginx.conf /etc/nginx/conf.d/default.conf

COPY --from=builder /app/dist /usr/share/nginx/html

# wget is in the alpine base; --spider hits the URL without downloading.
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD wget --quiet --tries=1 --spider http://localhost/healthz || exit 1

EXPOSE 80
