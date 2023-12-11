#version 300 es

precision highp float;

in vec3 v_normal;

uniform vec3 u_reverse_light_direction;
uniform vec4 u_color;

out vec4 out_color;

void main() {
  vec3 normal = normalize(v_normal);
  float light = dot(normal, u_reverse_light_direction);
  light /= 2.3;
  light += 0.5;

  // out_color = vec4(1.0, 0.3, 0.8, 1.0);
  out_color = u_color;
  out_color.rgb *= light;
}
