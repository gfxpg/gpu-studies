#version 300 es

precision mediump float;

// RGB color passed from the vertex shader, which receives it as an attribute.
in vec3 v_color;

out vec4 out_color;

void main() {
  out_color = vec4(v_color, 1.0);
}
