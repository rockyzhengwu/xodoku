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

The current web app is deployed best as a static Next.js export. Sudoku solving,
generation, and OCR run in the browser through WebAssembly, Web Workers, and
ONNX Runtime, so no server runtime is required.

Use these Cloudflare Pages settings:

- Root directory: repository root
- Build command: `pnpm build`
- Build output directory: `apps/web/out`
- Environment variables: `NODE_VERSION=24.8.0`, `PNPM_VERSION=10.26.2`

Cloudflare Pages must also be able to run the Rust WebAssembly build step. If
the Pages build image does not have Rust and `wasm-pack`, either install them in
the build command before `pnpm build`, or build in CI and upload `apps/web/out`
to Pages.
