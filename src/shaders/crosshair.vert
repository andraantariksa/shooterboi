#version 450

layout(std140, binding = 0) uniform rendering_info {
    vec3 reso_time;
    vec3 cam_pos;
    vec3 cam_dir;
    vec2 fov_shootanim;
    uvec3 queuecount_raymarchmaxstep_aostep;
};

layout(location = 0) in vec2 pos;
layout(location = 1) in vec4 col_in;

layout(location = 0) out vec4 col_out;

void main() {
    col_out = col_in;
    gl_Position = vec4(pos / (reso_time.xy / 2.), 0., 1.);
}
