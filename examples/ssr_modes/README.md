# Server-Side Rendering Modes

This example shows the different "rendering modes" that can be used while server-side 
rendering an application:
1. **Synchronous**: Serve an HTML shell that includes `fallback` for any `Suspense`. Load data on the client, replacing `fallback` once they're loaded.
     - *Pros*: App shell appears very quickly: great TTFB (time to first byte).
     - *Cons*: Resources load relatively slowly; you need to wait for JS + Wasm to load before even making a request.
 2. **Out-of-order streaming**: Serve an HTML shell that includes `fallback` for any `Suspense`. Load data on the **server**, streaming it down to the client as it resolves, and streaming down HTML for `Suspense` nodes.
     - *Pros*: Combines the best of **synchronous** and **`async`**, with a very fast shell and resources that begin loading on the server.
     - *Cons*: Requires JS for suspended fragments to appear in correct order. Weaker meta tag support when it depends on data that's under suspense (has already streamed down `<head>`)
 3. **In-order streaming**: Walk through the tree, returning HTML synchronously as in synchronous rendering and out-of-order streaming until you hit a `Suspense`. At that point, wait for all its data to load, then render it, then the rest of the tree.
     - *Pros*: Does not require JS for HTML to appear in correct order.
     - *Cons*: Loads the shell more slowly than out-of-order streaming or synchronous rendering because it needs to pause at every `Suspense`. Cannot begin hydration until the entire page has loaded, so earlier pieces
       of the page will not be interactive until the suspended chunks have loaded.
 4. **`async`**: Load all resources on the server. Wait until all data are loaded, and render HTML in one sweep.
     - *Pros*: Better handling for meta tags (because you know async data even before you render the `<head>`). Faster complete load than **synchronous** because async resources begin loading on server.
     - *Cons*: Slower load time/TTFB: you need to wait for all async resources to load before displaying anything on the client.

## Server Side Rendering with `cargo-leptos`
`cargo-leptos` is now the easiest and most featureful way to build server side rendered apps with hydration. It provides automatic recompilation of client and server code, wasm optimisation, CSS minification, and more! Check out more about it [here](https://github.com/akesson/cargo-leptos)

1. Install cargo-leptos
```bash
cargo install --locked cargo-leptos
``` 
2. Build the site in watch mode, recompiling on file changes
```bash
cargo leptos watch
```

Open browser on [http://localhost:3000/](http://localhost:3000/)

3. When ready to deploy, run
```bash
cargo leptos build --release
```

## Server Side Rendering without cargo-leptos
To run it as a server side app with hydration, you'll need to have wasm-pack installed.

0. Edit the `[package.metadata.leptos]` section and set `site-root` to `"."`. You'll also want to change the path of the `<StyleSheet / >` component in the root component to point towards the CSS file in the root. This tells leptos that the WASM/JS files generated by wasm-pack are available at `./pkg` and that the CSS files are no longer processed by cargo-leptos. Building to alternative folders is not supported at this time. You'll also want to edit the call to `get_configuration()` to pass in `Some(Cargo.toml)`, so that Leptos will read the settings instead of cargo-leptos. If you do so, your file/folder names cannot include dashes.
1. Install wasm-pack
```bash
cargo install wasm-pack
```
2. Build the Webassembly used to hydrate the HTML from the server
```bash
wasm-pack build --target=web --debug --no-default-features --features=hydrate
```
3. Run the server to serve the Webassembly, JS, and HTML 
```bash
cargo run --no-default-features --features=ssr
```

