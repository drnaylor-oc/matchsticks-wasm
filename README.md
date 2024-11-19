# Matchsticks and Numbers: WebAssembly library

This small Rust code is designed to at least obfuscate the Matchsticks algorithm to try to deter people from just looking it up, while
allowing it all to run server side.

This has been generated from the Rust WebAssembly template.

## 🚴 Usage

### Test Rust code natively

This targets the `aarch64-apple-darwin` target, if you need a different target, alter the test command within the script accordingly.

```
./unit_test_lib.sh
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### Testing/deploying the site

Uses Node 22.

From `site`, for serving the development files, or building the production site

```
npm run serve
npm run build
```