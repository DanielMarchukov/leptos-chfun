# leptos-chfun

## Local development

### Prerequisites

- Rust (stable) with the `wasm32-unknown-unknown` target:
  `rustup target add wasm32-unknown-unknown`
- [`cargo-leptos`](https://github.com/leptos-rs/cargo-leptos):
  `cargo install --locked cargo-leptos`

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
docker run --rm -p 3000:3000 leptos-chfun
```

Open `http://localhost:3000`.
