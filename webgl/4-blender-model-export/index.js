window.addEventListener('load', async () => {
  await wasm_bindgen('./pkg/blender_model_export_bg.wasm');

  const canvas = document.querySelector('canvas');

  wasm_bindgen.load_gl(canvas);

  //const obj = await (await fetch('cube.obj')).text();

  //const parsed = wasm_bindgen.load_vertices(obj, 0);

  //console.log(parsed);
});
