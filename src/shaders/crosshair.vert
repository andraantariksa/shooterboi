#version 450

layout(std140, binding = 0) uniform rendering_info {
    vec3 reso_time;
    vec3 cam_pos;
    vec3 cam_dir;
    vec2 fov_shootanim;
    uvec3 queuecount_raymarchmaxstep_aostep;
    vec4 crosshair_color;
    vec4 crosshair_inner_outer;
};

layout (location = 0) in vec2 pos;

void main() {
    gl_Position = vec4(pos.x / (reso_time.x / 2.0), pos.y / (reso_time.y / 2.0), 0., 0.);
}
