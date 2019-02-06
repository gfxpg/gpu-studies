window.addEventListener('load', async () => {
  await wasm_bindgen('./pkg/blender_model_export_bg.wasm');

  wasm_bindgen.greet('f');
});
