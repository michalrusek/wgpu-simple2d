#version 450

layout(location=0) in vec3 v_position;
layout(location=1) in vec2 v_tex_coords;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

layout(location=0) out vec4 f_color;

void main() {
    vec4 object_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);
    f_color = object_color;
}