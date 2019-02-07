#version 300 es
 
in vec4 a_position;
in vec3 a_color;

uniform mat4 u_world_transform;

out vec3 v_color;
 
void main() {
  v_color = a_color;
  gl_Position = (u_world_transform * a_position);
}
