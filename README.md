# Xodoku

Xodoku is a Rust-powered Sudoku player, generator, solver, and image scanner.
The web application is available at [xodoku.com](https://xodoku.com).

## Repository Layout

- [`crates/sudoku-rs`](./crates/sudoku-rs): Sudoku model, solver, and generator.
- [`crates/sudoku-wasm`](./crates/sudoku-wasm): WebAssembly bindings for the Rust core.
- [`apps/web`](./apps/web): Next.js player and scanner using WebAssembly and ONNX Runtime.

The layout leaves room for additional applications such as `apps/mobile`.

## Prerequisites

- Rust 1.95.0 with `rustfmt` and `clippy`
- Node.js 24.8.0
- pnpm 10.26.2
- wasm-pack 0.12.1

## Development

Install JavaScript dependencies and run the web application:

```sh
pnpm install
pnpm dev
```

`pnpm dev` builds the WebAssembly bindings before starting Next.js. Generated
bindings are written to `apps/web/app/wasm` and are not committed.

## Checks

Run the complete local verification suite:

```sh
pnpm check
```

Individual commands are available for `pnpm fmt:check`, `pnpm lint`,
`pnpm test`, `pnpm wasm:build`, and `pnpm web:build`.

## Deployment Build

The deployment environment must install Rust, wasm-pack, Node.js, and pnpm.
Build the application with:

```sh
pnpm wasm:build
pnpm web:build
```

## Cloudflare Pages

The web app is deployed as a static Next.js export. Sudoku solving, generation,
and OCR run in the browser through WebAssembly, Web Workers, and ONNX Runtime,
so no server runtime is required.

Deploy through GitHub Actions and Cloudflare Pages Direct Upload. This keeps the
Rust and `wasm-pack` build in CI instead of Cloudflare Pages' build image.

Required GitHub secrets:

- `CLOUDFLARE_API_TOKEN`
- `CLOUDFLARE_ACCOUNT_ID`

The workflow in `.github/workflows/deploy-cloudflare-pages.yml` builds the app
with `pnpm build` and uploads `apps/web/out`:

```sh
wrangler pages deploy apps/web/out --project-name=xodoku --branch=main
```

Use a Cloudflare Pages Direct Upload project named `xodoku`, or change the
workflow `--project-name` to match the existing Pages project.
