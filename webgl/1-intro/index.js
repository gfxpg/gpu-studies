import { glCreateProgram, glLoadAttrib } from '../shared/gl.0.js';

const vertices = new Float32Array([
  // coordinates (x, y) go from -1 to 1, 0 is the center

  // first triangle
  -0.4, -0.9,
  -0.2, 0.2,
  -0.8, -0.7,

  // second triangle
  0, -0.3,
  0.3, 0.7,
  0.8, 0.3
]);

const colors = new Uint8Array([
  // first triangle
  124, 236, 132,
  124, 236, 188,
  124, 228, 236,

  // second triangle
  132, 124, 236,
  188, 124, 236,
  236, 124, 228
]);

async function main() {
  const gl = document.querySelector('canvas').getContext('webgl2'); 

  const program = await glCreateProgram({ gl,
    vertexShaderUrl: 'v-shader.glsl', fragmentShaderUrl: 'f-shader.glsl' });

  const va = gl.createVertexArray();
  gl.bindVertexArray(va);

  glLoadAttrib({ gl, program, attrib: 'a_position', contents: vertices,
    size: 2 /* (x, y) */, type: gl.FLOAT, normalize: false });

  glLoadAttrib({ gl, program, attrib: 'a_color', contents: colors,
    size: 3 /* (r, g, b) */, type: gl.UNSIGNED_BYTE, normalize: true });

  gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
  gl.clearColor(0, 0, 0, 0);
  gl.clear(gl.COLOR_BUFFER_BIT);

  gl.useProgram(program);

  gl.drawArrays(gl.TRIANGLES, 0 /* offset */, vertices.length / 2);
}

main();
