#version 450

#define EPS 0.0001
#define MAX_DISTANCE 100.0
#define MAX_QUEUE 100

#define SHAPE_TYPE_NONE 0
#define SHAPE_TYPE_BOX 1
#define SHAPE_TYPE_SPHERE 2
#define SHAPE_TYPE_CYLINDER 3
#define SHAPE_TYPE_SWORDMAN 4
#define SHAPE_TYPE_GUNMAN 5

#define MATERIAL_GREEN 0
#define MATERIAL_YELLOW 1
#define MATERIAL_WHITE 2
#define MATERIAL_BLACK 3
#define MATERIAL_CHECKER 4
#define MATERIAL_RED 5
#define MATERIAL_ORANGE 6

#define DEBUG_POSITION 0

struct RenderQueue
{
    vec3 position;
    vec3 scale;
    vec3 rotation;
    vec4 shape_data1;
    vec4 shape_data2;
    uvec4 shape_type_materials_id;
};

layout(std140, binding = 0) uniform rendering_info {
    vec3 reso_time;
    vec3 cam_pos;
    vec3 cam_dir;
    vec2 fov_shootanim;
    uvec3 queuecount_raymarchmaxstep_aostep;
};

#ifdef IS_WEB
layout(std140, binding = 1) uniform render_queue {
#else
layout(std430, binding = 1) readonly buffer render_queue {
#endif
    RenderQueue queue[70];
};

layout(binding = 2) uniform sampler checker_sampler;
layout(binding = 3) uniform texture2D checker_texture;

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
};

Distance sd_union(Distance d1, Distance d2)
{
    return d1.distance < d2.distance ? d1 : d2;
}

Distance sd_intersect(Distance d1, Distance d2)
{
    return d1.distance > d2.distance ? d1 : d2;
}

mat3 rot_z(float rad)
{
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

float smin(float a, float b, float k)
{
    float h = clamp(0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
    return mix(b, a, h) - k * h * (1.0 - h);
}

float sd_capsule_line(vec3 p, vec3 a, vec3 b, float r)
{
    vec3 pa = p - a, ba = b - a;
    float h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h) - r;
}

float sd_plane(vec3 pos, vec3 n, float h)
{
    return dot(pos, n) + h;
}

float sd_sphere(vec3 pos, float rad)
{
    return length(pos) - rad;
}

float sd_round_box(vec3 p, vec3 b, float r)
{
    vec3 q = abs(p) - b;
    return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0) - r;
}

float sd_box(vec3 pos, vec3 size)
{
    vec3 d = abs(pos) - size;
    return max(d.x, max(d.y, d.z));
}

float sd_capsule(vec3 p, vec3 a, vec3 b, float r)
{
    vec3 pa = p - a, ba = b - a;
    float h = clamp( dot(pa,ba)/dot(ba,ba), 0.0, 1.0 );
    return length( pa - ba*h ) - r;
}

// Arm - Common
const float arm_rad = .4;
const float arm_length = 2.3;
const vec3 arm_r_or = vec3(1.1, 1.6, 0.);

Distance sd_gunman(vec3 p, float shootanim, uint material_man, uint material_gun)
{
    // Head
    float d = sd_sphere(p - vec3(0., 3.1, 0.), 1.);
    // Body
    d = min(d, sd_round_box(p, vec3(.3, 1.4, .4), .7));
    // Arm Left
    const vec3 arm_l_or = vec3(-1.1, 1.4, 0.);
    const vec3 arm_l_v = vec3(0., -1., 0.);
    d = min(d, sd_capsule(p, arm_l_or, arm_l_or + arm_l_v * arm_length, arm_rad));
    // Arm Right
    const vec3 arm_r_1_end = arm_r_or + vec3(0., 0., arm_length / 2.);
    //// First joint
    d = min(d, sd_capsule(p, arm_r_or, arm_r_1_end, arm_rad));
    //// Second joint
    const vec3 arm_r_2_v = vec3(0., 0., 1.) * rot_x(shootanim);
    const vec3 arm_r_2_end = arm_r_1_end + arm_r_2_v * (arm_length / 2.);
    // Arm Left
    d = min(d, sd_capsule(p, arm_r_1_end, arm_r_2_end, arm_rad));
    // Leg symetric
    const float leg_length = 2.5;
    const vec3 leg_or = vec3(.5, -2., .0);
    const vec3 leg_v = vec3(0., -1., 0.);
    vec3 p_leg = p;
    p_leg.x = abs(p_leg.x);
    d = min(d, sd_capsule(p_leg, leg_or, leg_or + leg_v * leg_length, .4));

    const mat3 rot_shootanim_neg = rot_x(-shootanim);


    // Pistol
    const vec3 p_pistol = (p - arm_r_2_end) * rot_shootanim_neg;
    const float holder_y = .4;
    const float holder_z = .2;
    float e = sd_box(p_pistol - vec3(0., arm_rad, 0.), vec3(.1, holder_y, holder_z));

    const float upper_z = .8;
    const float upper_offset_z = -.1;
    e = min(e, sd_box(p_pistol - vec3(0., arm_rad + holder_y, upper_z - holder_z + upper_offset_z), vec3(.1, .2, upper_z)));

    return sd_union(Distance(d, material_man), Distance(e, material_gun));
}

Distance sd_swordman(vec3 p, float swordanim, uint material_man, uint material_sword)
{
    // Head
    float d = sd_sphere(p - vec3(0., 3.1, 0.), 1.);
    // Body
    d = min(d, sd_round_box(p, vec3(.3, 1.4, .4), .7));
    // Arm Left
    const vec3 arm_l_or = vec3(-1.1, 1.4, 0.);
    const vec3 arm_l_v = vec3(0., -1., 0.);
    d = min(d, sd_capsule(p, arm_l_or, arm_l_or + arm_l_v * arm_length, arm_rad));
    // Arm right
    const vec3 arm_r_1_end = arm_r_or + vec3(0., 0., arm_length / 2.);
    //// First joint
    d = min(d, sd_capsule(p, arm_r_or, arm_r_1_end, arm_rad));
    //// Second joint
    const vec3 arm_r_2_v = vec3(0., 0., 1.) * rot_x(swordanim);
    const vec3 arm_r_2_end = arm_r_1_end + arm_r_2_v * (arm_length / 2.);
    // Arm Left
    d = min(d, sd_capsule(p, arm_r_1_end, arm_r_2_end, arm_rad));
    // Leg symetric
    const float leg_length = 2.5;
    const vec3 leg_or = vec3(.5, -2., .0);
    const vec3 leg_v = vec3(0., -1., 0.);
    vec3 leg_p = p;
    leg_p.x = abs(leg_p.x);
    d = min(d, sd_capsule(leg_p, leg_or, leg_or + leg_v * leg_length, .4));

    // Sword
    const vec3 p_sword = (p - arm_r_2_end) * rot_x(-swordanim - 0.4);
    const float holder_y = 2.8;
    float e = sd_box(p_sword - vec3(0., holder_y * .7, 0.), vec3(.1, holder_y, .2));
    e = min(e, sd_box(p_sword  - vec3(0., holder_y * .15, 0.), vec3(.5, .1, .4)));

    return sd_union(Distance(d, material_man), Distance(e, material_sword));
}

float sd_cylinder(vec3 p, vec3 a, vec3 b, float r)
{
    vec3  ba = b - a;
    vec3  pa = p - a;
    float baba = dot(ba,ba);
    float paba = dot(pa,ba);
    float x = length(pa*baba-ba*paba) - r*baba;
    float y = abs(paba-baba*0.5)-baba*0.5;
    float x2 = x*x;
    float y2 = y*y*baba;
    float d = (max(x,y)<0.0)?-min(x2,y2):(((x>0.0)?x2:0.0)+((y>0.0)?y2:0.0));
    return sign(d)*sqrt(abs(d))/baba;
}

vec3 repeat(vec3 pos, vec3 c)
{
    return mod(pos + 0.5 * c, c) - 0.5 * c;
}

Distance scene_dist(vec3 pos)
{
    Distance m = Distance(sd_plane(pos, vec3(0., 1., 0.), 0), 4);

    for (uint i = 0u; i < queuecount_raymarchmaxstep_aostep.x; i++) {
        switch (queue[i].shape_type_materials_id.x) {
            case SHAPE_TYPE_BOX:
                m = sd_union(m,
                    Distance(
                        sd_box(
                            pos - queue[i].position,
                            queue[i].shape_data1.xyz),
                            queue[i].shape_type_materials_id.y));
                break;
            case SHAPE_TYPE_SPHERE:
                m = sd_union(m,
                        Distance(
                            sd_sphere(
                                pos - queue[i].position,
                                queue[i].shape_data1.x),
                                queue[i].shape_type_materials_id.y));
                break;
            case SHAPE_TYPE_CYLINDER:
                m = sd_union(m,
                    Distance(
                        sd_cylinder(
                            pos - queue[i].position,
                            queue[i].shape_data1.xyz,
                            queue[i].shape_data2.xyz,
                            queue[i].shape_data1.w),
                            queue[i].shape_type_materials_id.y));
                break;
            case SHAPE_TYPE_GUNMAN:
                {
                    const float s = 0.2;
                    Distance d = sd_gunman(
                        (pos - queue[i].position) / s * rot_y(queue[i].shape_data2.y),
                        queue[i].shape_data2.x,
                        queue[i].shape_type_materials_id.y,
                        queue[i].shape_type_materials_id.z);
                    d.distance *= s;
                    m = sd_union(m, d);
                }
                break;
            case SHAPE_TYPE_SWORDMAN:
                {
                    const float s = 0.2;
                    Distance d = sd_swordman(
                        (pos - queue[i].position) / s * rot_y(queue[i].shape_data2.y),
                        queue[i].shape_data2.x,
                        queue[i].shape_type_materials_id.y,
                        queue[i].shape_type_materials_id.z);
                    d.distance *= s;
                    m = sd_union(m, d);
                }
                break;
            default:
                break;
        }
    }

    return m;
}

vec3 get_normal(vec3 pos)
{
    float d = scene_dist(pos).distance;
    vec2 eps = vec2(EPS, 0.);
    
    vec3 normal = vec3(
        scene_dist(pos + eps.xyy).distance - d,
        scene_dist(pos + eps.yxy).distance - d,
        scene_dist(pos + eps.yyx).distance - d);
    
    return normalize(normal);
}

Distance ray_march(vec3 ray_origin, vec3 ray_dir)
{
    float d = 0.0;
    uint mat_id = 0;
    vec3 current_pos = vec3(0);

    for (uint i = 0u; i < queuecount_raymarchmaxstep_aostep.y; i++) {
        current_pos = ray_origin + d * ray_dir;
        Distance closest_distance = scene_dist(current_pos);

        if (abs(closest_distance.distance) < 0.0001 || d >= 100.0) {
            break;
        }

        d += closest_distance.distance;
        mat_id = closest_distance.materialId;
    }

    return Distance(d, mat_id);
}

float soft_shadow(in vec3 ro, in vec3 rd, float mint, float maxt, float k)
{
    float res = 1.0;
    float ph = 1e10;
    for (float t = mint; t < maxt; )
    {
        Distance h = scene_dist(ro + rd * t);
        if (h.distance < 0.001)
        return 0.0;
        float y = h.distance * h.distance / (2.0 * ph);
        float d = sqrt(h.distance * h.distance - y * y);
        res = min(res, k * d / max(0.0, t - y));
        ph = h.distance;
        t += h.distance;
    }
    return res;
}

float ambient_ocl(in vec3 pos, in vec3 nor)
{
    float occ = 0.0;
    float sca = 1.0;
    for (uint i = 0; i < queuecount_raymarchmaxstep_aostep.z; i++)
    {
        float h = 0.001 + 0.15 * float(i) / 4.0;
        Distance d = scene_dist(pos + h * nor);
        occ += (h - d.distance) * sca;
        sca *= 0.95;
    }
    return clamp(1.0 - 1.5 * occ, 0.0, 1.0);
}

float light(vec3 n, vec3 lp, vec3 l)
{
    return clamp(dot(n, normalize(l - lp)), 0, 1);
}

vec3 lookat(vec2 uv, vec3 pos, vec3 dir, vec3 w_up)
{
    vec3 right = normalize(cross(w_up, dir));
    vec3 up = normalize(cross(right, dir));
    return normalize(uv.x * right + uv.y * up + dir * 2.0);
}

vec3 ray_view_dir(vec2 size, vec2 coord) {
    vec2 xy = coord - size / 2.0;
    float z = size.y / tan(fov_shootanim.x / 2.0);
    return normalize(vec3(xy, z));
}

mat4 view_matrix(vec3 pos, vec3 dir, vec3 world_up) {
    vec3 right = normalize(cross(dir, world_up));
    vec3 up = normalize(cross(dir, right));
    return mat4(
        vec4(right, 0.0),
        vec4(up, 0.0),
        vec4(dir, 0.0),
        vec4(0.0, 0.0, 0.0, 1));
}

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

const vec3 skycolor = vec3(113, 188, 225) / 255.0;
const vec3 world_up = vec3(0.0, 1.0, 0.0);

void main()
{
    vec3 ray_view_dir = ray_view_dir(reso_time.xy, gl_FragCoord.xy);
    mat4 view_to_world = view_matrix(cam_pos, cam_dir, world_up);
    vec3 ray_world_dir = (view_to_world * vec4(ray_view_dir, 0.0)).xyz;

    Distance d = ray_march(cam_pos, ray_world_dir);

    vec3 ray_hit_pos = cam_pos + d.distance * ray_world_dir;
    vec3 normal = get_normal(ray_hit_pos);

    vec3 checker_texture_xz = texture(sampler2D(checker_texture, checker_sampler), ray_hit_pos.xz).rgb;
    vec3 checker_texture_xy = texture(sampler2D(checker_texture, checker_sampler), ray_hit_pos.xy).rgb;
    vec3 checker_texture_yz = texture(sampler2D(checker_texture, checker_sampler), ray_hit_pos.yz).rgb;

    if (d.distance > MAX_DISTANCE - EPS) {
        outColor = vec4(skycolor, 1.0);
        return;
    }

    vec3 col = vec3(46., 209., 162.) / 255.;
    switch (d.materialId) {
        case MATERIAL_RED:
            col = vec3(1., 0., 0.);
            break;
        case MATERIAL_YELLOW:
            col = vec3(230., 255., 110.) / 255.;
            break;
        case MATERIAL_WHITE:
            col = vec3(1.);
            break;
        case MATERIAL_CHECKER:
            col = checker_texture_yz * abs(normal.x) + checker_texture_xy * abs(normal.z) + checker_texture_xz * abs(normal.y);
            break;
        case MATERIAL_BLACK:
            col = vec3(0.);
            break;
        case MATERIAL_GREEN:
            col = vec3(0., 1., 0.);
            break;
        case MATERIAL_ORANGE:
            col = vec3(1.0, 0.30, 0.0);
            break;
        default:
            break;
    }

    const vec3 ambient_light = skycolor * col * 0.5;
    vec3 color = ambient_light;
    color += blinn_phong(
        col,
        vec3(1.0),
        10.0,
        ray_hit_pos,
        cam_pos,
        vec3(0.0, 10.0, 0.0),
        vec3(0.4),
        normal);
#if DEBUG_POSITION == 1
    color += ray_hit_pos * 0.3;
#endif
    color *= ambient_ocl(ray_hit_pos, normal);
    outColor = vec4(color, 1.0);
}
