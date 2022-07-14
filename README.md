# Vite + Svelte + TS + Rocket

This is a barebones template for a svelte-ts, rocket SPA project. You can modify it to utilise SSG through vite or SSR using SvelteKit.
Make sure to make a PR for your SSG or SSR solution ;).

## Start

Install `cargo-watch` to use the live-preview

```bash
cargo install cargo-watch
```

There are three scripts at your disposal:

```bash
npm run serve
```

builds the vite project in watch mode and starts the rocket server in release mode. The release build of the rocket server, serves the static files from `dist`.

```bash
npm run dev
```

starts the vite dev-server + rocket in debug-mode. Rocket doesn't serve the static files and only acts as a backend server. There are two servers running so check the console output for their bound ports.

```bash
npm run build
```

builds the vite project and the rocket server in release mode. Rust files are located in `target` and vite files in `dist`.

This command starts `vite build` in `watch` mode and `cargo watch -x run -w server`.
Cargo is configured in a way that all rust code is in the `server`-directory instead of `src`.

## How this project was setup

First a simple svelte-ts-vite project was setup:

```bash
$ npm init vite@latest
<select svelte-ts>
```

Then rocket was added as a dependency and the build scrips were added to `package.json`.
The npm-dependency `concurrently` had to be added to allow both watch scripts to run in parallel.

## Branches

I will add a new branch for each feature I decide to add like _tailwind css_.
