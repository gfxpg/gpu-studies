export async function glCreateProgram({ gl, vertexShaderUrl, fragmentShaderUrl }) {
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

export function glLoadAttrib({ gl, program, attrib, contents, size, type, normalize }) {
  const attribLocation = gl.getAttribLocation(program, attrib),
        attribBuffer   = gl.createBuffer();

  gl.bindBuffer(gl.ARRAY_BUFFER, attribBuffer);
  gl.bufferData(gl.ARRAY_BUFFER, contents, gl.STATIC_DRAW);

  gl.enableVertexAttribArray(attribLocation);
  gl.vertexAttribPointer(attribLocation, size, type, normalize, 0 /* stride */, 0 /* offset */);
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
