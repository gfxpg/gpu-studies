# Blender Model Export 

Building the WebAssembly package:

```
wasm-pack build --target no-modules
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
