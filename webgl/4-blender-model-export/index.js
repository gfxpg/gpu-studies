window.addEventListener('load', async () => {
  await wasm_bindgen('./pkg/blender_model_export_bg.wasm');

  const obj = await (await fetch('cube.obj')).text();

  const parsed = wasm_bindgen.load_vertices(obj, 0);

  console.log(parsed);
});
