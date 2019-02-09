import { rad } from '../shared/transform.0.js';
import MouseControls from '../shared/mousecontrols.0.js';

function setupRangeInput(id, onInput) {
  document.getElementById(id).addEventListener('input', ({ target: { value } }) => {
    document.getElementById(`${id}-val`).textContent = value;
    onInput(parseFloat(value));
  });
}

window.addEventListener('load', async () => {
  await wasm_bindgen('./pkg/sine_wave_bg.wasm');

  const canvas = document.querySelector('canvas'),
        status = document.getElementById('status');

  status.textContent = 'Loading...';

  const renderer = wasm_bindgen.Renderer.new(canvas);

  renderer.instantiate();
  renderer.render();

  setupRangeInput('scale', renderer.update_scale.bind(renderer));
  setupRangeInput('amplitude', renderer.update_amplitude.bind(renderer));
  setupRangeInput('phase', renderer.update_phase.bind(renderer));
  setupRangeInput('freq', renderer.update_freq.bind(renderer));
  new MouseControls(canvas, renderer.update_rotation.bind(renderer));

  status.textContent = 'Hold LMB and move your mouse across the canvas to rotate the figure.';
});
