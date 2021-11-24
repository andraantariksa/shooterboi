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

layout(location = 0) out vec4 outColor;

void main()
{
    outColor = crosshair_color;
}
