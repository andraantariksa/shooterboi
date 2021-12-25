#define SENTINEL_IDX 9999

#define EPS 0.0001
#define MAX_DISTANCE 500.0
#define MAX_QUEUE 100

#define SHAPE_TYPE_NONE 0
#define SHAPE_TYPE_BOX 1
#define SHAPE_TYPE_SPHERE 2
#define SHAPE_TYPE_CYLINDER 3
#define SHAPE_TYPE_SWORDMAN 4
#define SHAPE_TYPE_GUNMAN 5

#define DEBUG_POSITION 0

#define PI 3.1415
#define PI2 6.2832

struct RenderQueue
{
    vec3 position;
    vec3 scale;
    mat4 rotation;
    vec4 shape_data1;
    vec4 shape_data2;
    uvec4 shape_type_materials_id;
};

layout(std140, binding = 0) uniform rendering_info {
    vec3 reso_time;
    vec3 cam_pos;
    vec3 cam_dir;
    vec2 fov_shootanim;
    uvec4 queuecount_raymarchmaxstep_aostep_background_type;
};

#ifdef IS_WEB
layout(std140, binding = 1) uniform render_queue {
#else
layout(std430, binding = 1) readonly buffer render_queue {
    #endif
    RenderQueue queue[70];
};

#define MATERIAL_GREEN 0
#define MATERIAL_YELLOW 1
#define MATERIAL_WHITE 2
#define MATERIAL_BLACK 3
#define MATERIAL_CHECKER 4
#define MATERIAL_RED 5
#define MATERIAL_ORANGE 6
#define MATERIAL_CRATE 7
#define MATERIAL_PEBBLES 8
#define MATERIAL_COBBLESTONE_PAVING 9
#define MATERIAL_CONTAINER 10
#define MATERIAL_TARGET 11
#define MATERIAL_GRASS 12
#define MATERIAL_STONE_WALL 13
#define MATERIAL_BUILDING 14
#define MATERIAL_TARGET_DIMMED 15
#define MATERIAL_TREE_BARK 16
#define MATERIAL_LEAVES 17
#define MATERIAL_RGBA_NOISE_MEDIUM 18
#define MATERIAL_GRAY_NOISE_SMALL 19
#define MATERIAL_ASPHALT 20

layout(binding = 2) uniform sampler common_sampler;
layout(binding = 3) uniform texture2D checker_texture;
layout(binding = 4) uniform texture3D noise_vol_gray_texture;
layout(binding = 16) uniform texture2D crate_texture;
layout(binding = 6) uniform texture2D pebbles_texture;
layout(binding = 7) uniform texture2D abstract3_texture;
layout(binding = 8) uniform texture2D cobblestone_paving_texture;
layout(binding = 9) uniform texture2D container_texture;
layout(binding = 10) uniform texture2D target_texture;
layout(binding = 11) uniform texture2D grass_texture;
layout(binding = 12) uniform texture2D stone_wall_texture;
layout(binding = 13) uniform texture2D tree_bark_texture;
layout(binding = 14) uniform texture2D leaves_texture;
layout(binding = 15) uniform texture2D rgba_noise_medium;
layout(binding = 5) uniform texture2D gray_noise_small_texture;
layout(binding = 17) uniform texture2D asphalt_texture;

layout(location = 0) out vec4 outColor;

struct RayHit
{
    vec3 pos;
    float dist;
};

struct Distance
{
    float distance;
    uint materialId;
    uint idx;
};

Distance scene_dist(vec3 pos);

// Constant

const float tau = 6.283185;
const float pi = 3.14159;

// Main

vec3 get_normal(vec3 pos) {
    const vec2 e = vec2(1., -1.);

    vec3 dfa = e.xyy * scene_dist(pos + e.xyy * EPS).distance;
    vec3 dfb = e.yxy * scene_dist(pos + e.yxy * EPS).distance;
    vec3 dfc = e.yyx * scene_dist(pos + e.yyx * EPS).distance;
    vec3 dfd = e.xxx * scene_dist(pos + e.xxx * EPS).distance;

    return normalize(dfa + dfb + dfc + dfd);
}

Distance ray_march(vec3 ray_origin, vec3 ray_dir) {
    float d = 0.0;
    uint mat_id = 0;
    uint idx = 0;
    vec3 current_pos = vec3(0);

    for (uint i = 0u; i < queuecount_raymarchmaxstep_aostep_background_type.y; i++) {
        current_pos = ray_origin + d * ray_dir;
        Distance closest_distance = scene_dist(current_pos);

        if (abs(closest_distance.distance) < EPS || d >= MAX_DISTANCE) {
            break;
        }

        d += closest_distance.distance;
        mat_id = closest_distance.materialId;
        idx = closest_distance.idx;
    }

    return Distance(d, mat_id, idx);
}

// Shading

vec3 blinn_phong(
vec3 k_d,
vec3 k_s,
float alpha,
vec3 p,
vec3 eye,
vec3 light_pos,
vec3 light_intensity,
vec3 n)
{
    vec3 l = normalize(light_pos - p);
    vec3 v = normalize(eye - p);
    vec3 h = normalize(l + v);
    float diff = max(dot(l, n), 0.0);
    float spec = max(dot(h, n), 0.0);
    if (diff < 0.0) {
        // Light not visible from this point on the surface
        return vec3(0.0, 0.0, 0.0);
    }
    if (spec < 0.0) {
        // Light reflection in opposite direction as viewer, apply only diffuse
        // component
        return light_intensity * (k_d * diff);
    }
    return light_intensity * (k_d * diff + k_s * pow(spec, alpha));
}

float ambient_ocl(in vec3 pos, in vec3 nor) {
    float occ = 0.0;
    float sca = 1.0;
    for (uint i = 0; i < queuecount_raymarchmaxstep_aostep_background_type.z; i++)
    {
        float h = 0.001 + 0.15 * float(i) / 4.0;
        Distance d = scene_dist(pos + h * nor);
        occ += (h - d.distance) * sca;
        sca *= 0.95;
    }
    return clamp(1.0 - 1.5 * occ, 0.0, 1.0);
}

// Camera

vec3 ray_view_dir(vec2 size, vec2 coord) {
    vec2 xy = coord - size / 2.0;
    float z = size.y / tan(fov_shootanim.x / 2.0);
    return normalize(vec3(xy, z));
}

const vec3 WORLD_UP = vec3(0.0, 1.0, 0.0);

mat4 view_matrix(vec3 pos, vec3 dir) {
    vec3 right = normalize(cross(dir, WORLD_UP));
    vec3 up = normalize(cross(dir, right));
    return mat4(
    vec4(right, 0.0),
    vec4(up, 0.0),
    vec4(dir, 0.0),
    vec4(0.0, 0.0, 0.0, 1));
}

// Rotate

mat3 rot_z(float rad) {
    float s = sin(rad);
    float c = cos(rad);
    return mat3(
    c, -s, 0,
    s, c, 0,
    0, 0, 1);
}

mat3 rot_y(float rad) {
    float c = cos(rad);
    float s = sin(rad);
    return mat3(
    c, 0.0, -s,
    0.0, 1.0, 0.0,
    s, 0.0, c);
}

mat3 rot_x(float rad) {
    float c = cos(rad);
    float s = sin(rad);
    return mat3(
    1., 0., 0.,
    0., c, -s,
    0., s, c);
}

// SDF

Distance sd_union(Distance d1, Distance d2)
{
    return d1.distance < d2.distance ? d1 : d2;
}

Distance sd_intersect(Distance d1, Distance d2)
{
    return d1.distance > d2.distance ? d1 : d2;
}

float smin(float a, float b, float k)
{
    float h = clamp(0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
    return mix(b, a, h) - k * h * (1.0 - h);
}

vec3 repeat(vec3 pos, vec3 c)
{
    return mod(pos + 0.5 * c, c) - 0.5 * c;
}

// Fog

const vec3 FOG_COLOR = vec3(111., 106., 165.) / 255.;
const float FOG_DENSITY = .3;
vec3 mix_fog(vec3 color, float distance) {
    const float fog_intensity = exp(-FOG_DENSITY * max(distance - MAX_DISTANCE * .8, 0.));
    return mix(FOG_COLOR, color, fog_intensity);
}
