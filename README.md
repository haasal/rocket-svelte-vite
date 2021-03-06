# Vite + Svelte + TS + Rocket

This is a barebones template for a svelte-ts, rocket SPA project. You can modify it to utilise SSG through vite or SSR using SvelteKit.
Make sure to make a PR for your SSG or SSR solution ;).

## Start

Install `cargo-watch` to use the live-preview

```bash
cargo install cargo-watch
```

Clone the repository using [npm degit](https://www.npmjs.com/package/degit):

```bash
npm i -g degit
degit haasal/rocket-svelte-vite your_project_name
cd your_project_name
npm install
```

If you wish to use for example the tailwind branch use

```bash
degit haasal/rocket-svelte-vite#tailwind your_project_name
```

There are three important scripts at your disposal:

```bash
npm run serve
```

builds the vite project in watch mode and starts the rocket server in release mode. The release build of the rocket server, serves the static files from `dist`.

```bash
npm run dev:serve
```

starts the vite dev-server + rocket in debug-mode. Rocket doesn't serve the static files and only acts as a backend server. There are two servers running so check the console output for their bound ports.

```bash
npm run dev:rocket
```

runs only the rocket server in debug+watch mode.

```bash
npm run dev:vite
```

runs only the vite dev server in isolation.

```bash
npm run build
```

builds the vite project and the rocket server in release mode. Rust files are located in `target` and vite files in `dist`.

This command starts `vite build` in `watch` mode and `cargo watch -x run -w server`.
Cargo is configured in a way that all rust code is in the `server`-directory instead of `src`.

I might add new scripts in the future. So make sure to check out the package.json for any usefull scripts not listed here.
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
