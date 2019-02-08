import { rad } from '../shared/transform.0.js';
import MouseControls from '../shared/mousecontrols.0.js';

window.addEventListener('load', async () => {
  await wasm_bindgen('./pkg/textures_bg.wasm');

  const canvas = document.querySelector('canvas');
  const renderer = wasm_bindgen.Renderer.new(canvas);

  const textureReq = await fetch('cube.png');
  const textureBlob = await textureReq.blob();
  const texture = await window.createImageBitmap(textureBlob);

  renderer.instantiate_with_texture(texture);
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
