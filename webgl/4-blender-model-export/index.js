window.addEventListener('load', async () => {
  await wasm_bindgen('./pkg/blender_model_export_bg.wasm');

  const canvas = document.querySelector('canvas');
  const renderer = wasm_bindgen.Renderer.new(canvas);

  renderer.instantiate();
  renderer.render();
});
