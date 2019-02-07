import { rad } from '../shared/transform.0.js';
import MouseControls from '../shared/mousecontrols.0.js';

window.addEventListener('load', async () => {
  await wasm_bindgen('./pkg/blender_model_export_bg.wasm');

  const canvas = document.querySelector('canvas');
  const renderer = wasm_bindgen.Renderer.new(canvas);

  renderer.instantiate();
  renderer.render();

  new MouseControls(canvas, (rotX, rotY) => {
    renderer.set_rotation(rotX, rotY);
    renderer.render();
  });

  document.querySelector('#scale').addEventListener('input', ({ target: { value } }) => {
    renderer.set_scale(parseFloat(value));
    renderer.render();
  });
});
