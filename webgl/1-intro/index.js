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

  const program = await createProgram({ gl,
    vertexShaderUrl: 'v-shader.glsl', fragmentShaderUrl: 'f-shader.glsl' });

  const va = gl.createVertexArray();
  gl.bindVertexArray(va);

  /* Load vertices */

  const positionAttrib = gl.getAttribLocation(program, "a_position"),
        positionBuffer = gl.createBuffer();

  gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
  gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);

  gl.enableVertexAttribArray(positionAttrib);
  gl.vertexAttribPointer(positionAttrib,
    2,        /* size = 2 (x, y) */
    gl.FLOAT,
    false,    /* no normalization (see the color attribute pointer for an example of normalization) */
    0,        /* stride */
    0);       /* offset */

  /* Load colors */

  const colorAttrib = gl.getAttribLocation(program, "a_color"),
        colorBuffer = gl.createBuffer();

  gl.bindBuffer(gl.ARRAY_BUFFER, colorBuffer);
  gl.bufferData(gl.ARRAY_BUFFER, colors, gl.STATIC_DRAW);

  gl.enableVertexAttribArray(colorAttrib);
  gl.vertexAttribPointer(colorAttrib,
    3,        /* size = 3 (r, g, b) */
    gl.UNSIGNED_BYTE,
    true,     /* convert uint8 to float: 0 = 0.0, 255 = 1.0 */
    0,        /* stride */
    0);       /* offset */

  gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
  gl.clearColor(0, 0, 0, 0);
  gl.clear(gl.COLOR_BUFFER_BIT);

  gl.useProgram(program);

  gl.drawArrays(gl.TRIANGLES, 0 /* offset */, vertices.length / 2);
}

async function createProgram({ gl, vertexShaderUrl, fragmentShaderUrl }) {
  const program = gl.createProgram();

  const vertexShader = await getShader(gl, gl.VERTEX_SHADER, vertexShaderUrl),
        fragmentShader = await getShader(gl, gl.FRAGMENT_SHADER, fragmentShaderUrl);

  gl.attachShader(program, vertexShader);
  gl.attachShader(program, fragmentShader);
  gl.linkProgram(program);

  if (gl.getProgramParameter(program, gl.LINK_STATUS))
    return program;
 
  console.log(gl.getProgramInfoLog(program));
}

async function getShader(gl, type, url) {
  const source = await (await fetch(url)).text();
  const shader = gl.createShader(type);

  gl.shaderSource(shader, source);
  gl.compileShader(shader);

  if (gl.getShaderParameter(shader, gl.COMPILE_STATUS))
    return shader;
 
  console.log(gl.getShaderInfoLog(shader));
}

main();
