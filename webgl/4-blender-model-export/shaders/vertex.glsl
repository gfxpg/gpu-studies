#version 300 es
 
in vec4 a_position;
in vec3 a_color;

//uniform mat4 u_x_rotation;
//uniform mat4 u_y_rotation;
//uniform mat4 u_scale;

out vec3 v_color;
 
void main() {
  v_color = a_color;
  gl_Position = a_position; //(u_x_rotation * u_y_rotation * u_scale) * a_position;
}
