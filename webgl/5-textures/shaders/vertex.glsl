#version 300 es
 
in vec4 a_position;
in vec2 a_texcoord;

uniform mat4 u_world_transform;

out vec2 v_texcoord;
 
void main() {
  v_texcoord = a_texcoord;

  gl_Position = (u_world_transform * a_position);
}
