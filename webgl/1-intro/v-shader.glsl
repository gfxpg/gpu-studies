#version 300 es
 
in vec4 a_position;
in vec3 a_color;

// `out` is a _varying_ value passed to the fragment shader.
// It is _interpolated_, so we automagically get a gradient
// when we specify colors for each triangle vertex.
out vec3 v_color;
 
void main() {
  v_color = a_color;
  gl_Position = a_position;
}
