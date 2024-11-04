# Matchsticks and Numbers: WebAssembly library

This small Rust code is designed to at least obfuscate the Matchsticks algorithm to try to deter people from just looking it up, while
allowing it all to run server side.

This has been generated from the Rust WebAssembly template.

**This has been pinned to `rustc` 1.81.0, as Webpack cannot handle reference types in WebAssembly right now (enabled in Rust 1.82.0).**

## 🚴 Usage

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### Testing/deploying the site

From `site`, for serving the development files, or building the production site

```
npm run serve
npm run build
```