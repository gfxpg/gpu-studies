#version 300 es
 
in vec4 a_position;

uniform mat4 u_world_transform;
uniform float u_amplitude;
uniform float u_phase;
uniform float u_freq;

out vec2 v_texcoord;

void main() {
  vec4 pos = a_position;
  pos.y = u_amplitude * sin(u_freq * pos.x + u_phase);
  gl_Position = u_world_transform * pos;
}
