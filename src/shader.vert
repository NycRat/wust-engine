#version 300 es

precision highp float;

in vec4 a_pos;
in vec3 a_normal;
// in vec4 a_color;

uniform mat4 u_transformation;
uniform mat4 u_world_inverse_transposed;

// out vec4 v_color;
out vec3 v_normal;

void main() {
  gl_Position = a_pos * u_transformation ;
  v_normal = a_normal * mat3(u_world_inverse_transposed);
}
