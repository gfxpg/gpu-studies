import { rad } from '../shared/transform.0.js';
import MouseControls from '../shared/mousecontrols.0.js';

function setupRangeInput(id, onInput) {
  document.getElementById(id).addEventListener('input', ({ target: { value } }) => {
    document.getElementById(`${id}-val`).textContent = value;
    onInput(parseFloat(value));
  });
}

let animationEnabled = false;
let animationSpeed = rad(1.0);
let animationLastFpsUpdate = 0;
const animationTimes = [];

function animationUpdateFpsCounter() {
  // https://www.growingwiththeweb.com/2017/12/fast-simple-js-fps-counter.html
  const now = window.performance.now();
  while (animationTimes.length > 0 && animationTimes[0] <= now - 1000)
    animationTimes.shift();
  animationTimes.push(now);

  if (animationLastFpsUpdate < now - 1000) {
    document.getElementById('fps-val').textContent = animationTimes.length;
    animationLastFpsUpdate = now;
  }
}

function animate(renderer) {
  if (!animationEnabled) return;

  const phaseVal = document.getElementById('phase-val');
  let phase = parseFloat(phaseVal.textContent) + animationSpeed;
  if (phase > 6.28) phase = 0.0; // it's better ux if we limit the displayed value to 0..2pi

  phaseVal.textContent = phase.toFixed(2);
  document.getElementById('phase').value = phase;
  renderer.update_phase(phase);

  animationUpdateFpsCounter();
  window.requestAnimationFrame(() => animate(renderer));
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
  setupRangeInput('speed-deg', (val) => { animationSpeed = rad(val); });
  new MouseControls(canvas, renderer.update_rotation.bind(renderer));

  const animationToggle = document.getElementById('animate');

  animationToggle.addEventListener('click', () => {
    if (!animationEnabled) {
      animationToggle.textContent = 'Stop';
      window.requestAnimationFrame(() => animate(renderer));
    }
    else {
      animationToggle.textContent = 'Animate';
    }
    animationEnabled = !animationEnabled;
  });

  status.textContent = 'Hold LMB and move your mouse across the canvas to rotate the figure.';
});
