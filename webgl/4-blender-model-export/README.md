# Blender Model Export 

## Development

Building the WebAssembly package:

```
wasm-pack build --dev --target no-modules
```

Requires [Rust nightly](https://github.com/rust-lang/rustup.rs/blob/master/README.md#working-with-nightly-rust)
and [wasm-pack](https://rustwasm.github.io/wasm-pack/).

## Release build

```
wasm-pack build --release --target no-modules
```

Optionally:

```
minify pkg/blender_model_export.js -o pkg/blender_model_export.js
```

The `minify` command is provided by [babel-minify](https://github.com/babel/minify).
To install it globally, run:

```
npm i -g babel-minify
```
