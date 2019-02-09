#version 300 es
 
in vec4 a_position;

uniform mat4 u_world_transform;

out vec2 v_texcoord;

void main() {
  vec4 pos = a_position;
  pos.y = sin(pos.x * 8.0);
  gl_Position = u_world_transform * pos;
}
