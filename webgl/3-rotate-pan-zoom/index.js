import { glCreateProgram, glLoadAttrib } from '../shared/gl.0.js';
import { rad, matrotx3d, matroty3d } from '../shared/transform.0.js';
import MouseControls from '../shared/mousecontrols.0.js';

const vertices = new Float32Array([
  // x, y, z, from -1 to 1, 0 is the center
  
  // Note that triangles _have_ to have their vertices go in a counter-clockwise
  // direction; otherwise, they will be culled (not drawn).

  // front
  0.5, -0.4, -0.25,  // right bottom vertex
  0, 0.4, 0.25,      // top vertex
  -0.5, -0.4, -0.25, // left bottom vertex

  // left
  -0.5, -0.4, -0.25, // left bottom vertex
  0, 0.4, 0.25,      // top vertex
  0, -0.4, 0.25,     // center bottom vertex

  // right
  0, -0.4, 0.25,     // center bottom vertex
  0, 0.4, 0.25,      // top vertex
  0.5, -0.4, -0.25,  // right bottom vertex

  // bottom
  -0.5, -0.4, -0.25,
  0, -0.4, 0.25,
  0.5, -0.4, -0.25
]);

const colors = new Uint8Array([
  236, 188, 124,
  236, 188, 124,
  236, 188, 124,

  124, 236, 132,
  124, 236, 132,
  124, 236, 132,

  188, 124, 236,
  188, 124, 236,
  188, 124, 236,

  0, 0, 0,
  0, 0, 0,
  0, 0, 0
]);

async function main() {
  const canvas = document.querySelector('canvas');
  const gl = canvas.getContext('webgl2'); 

  const program = await glCreateProgram({ gl,
    vertexShaderUrl: 'v-shader.glsl', fragmentShaderUrl: 'f-shader.glsl' });

  const va = gl.createVertexArray();
  gl.bindVertexArray(va);

  glLoadAttrib({ gl, program, attrib: 'a_position', contents: vertices,
    size: 3 /* (x, y, z) */, type: gl.FLOAT, normalize: false });

  glLoadAttrib({ gl, program, attrib: 'a_color', contents: colors,
    size: 3 /* (r, g, b) */, type: gl.UNSIGNED_BYTE, normalize: true });

  gl.enable(gl.CULL_FACE); /* Don't draw back-facing triangles */
  gl.enable(gl.DEPTH_TEST);

  gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
  gl.clearColor(0, 0, 0, 0);
  gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

  gl.useProgram(program);

  redraw(gl, program, 0, 0);

  new MouseControls(canvas, (rotX, rotY) => {
    redraw(gl, program, rotX, rotY);
  });
}

function redraw(gl, program, rotX, rotY) {
  const xRotationUniform = gl.getUniformLocation(program, 'u_x_rotation');
  gl.uniformMatrix4fv(xRotationUniform, false /* must be false */, matrotx3d(rotX));

  const yRotationUniform = gl.getUniformLocation(program, 'u_y_rotation');
  gl.uniformMatrix4fv(yRotationUniform, false /* must be false */, matroty3d(rotY));

  gl.drawArrays(gl.TRIANGLES, 0 /* offset */, vertices.length / 3);
}

main();
