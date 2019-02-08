#version 300 es
 
in vec4 a_position;

uniform mat4 u_world_transform;

out vec2 v_texcoord;
 
void main() {
  gl_Position = (u_world_transform * a_position);
}
