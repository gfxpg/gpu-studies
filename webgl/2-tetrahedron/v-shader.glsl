#version 300 es
 
in vec4 a_position;
in vec3 a_color;

uniform mat4 u_x_rotation;
uniform mat4 u_y_rotation;
uniform mat4 u_z_rotation;

out vec3 v_color;
 
void main() {
  v_color = a_color;
  gl_Position = (u_x_rotation * u_y_rotation * u_z_rotation) * a_position;
}
