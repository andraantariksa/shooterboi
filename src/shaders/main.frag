#version 450

#define SENTINEL_IDX 9999

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
#define MATERIAL_CRATE 7
#define MATERIAL_PEBBLES 8

#define DEBUG_POSITION 0

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
    uvec3 queuecount_raymarchmaxstep_aostep;
};

#ifdef IS_WEB
layout(std140, binding = 1) uniform render_queue {
#else
layout(std430, binding = 1) readonly buffer render_queue {
#endif
    RenderQueue queue[70];
};

layout(binding = 2) uniform sampler common_sampler;
layout(binding = 3) uniform texture2D checker_texture;
layout(binding = 4) uniform texture3D noise_vol_gray_texture;
layout(binding = 5) uniform texture2D crate_texture;
layout(binding = 6) uniform texture2D pebbles_texture;

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
const vec3 arm_r_or = vec3(-1.1, 1.6, 0.);

Distance sd_gunman(vec3 p, float shootanim, uint material_man, uint material_gun, uint idx)
{
    // Head
    float d = sd_sphere(p - vec3(0., 3.1, 0.), 1.);
    // Body
    d = min(d, sd_round_box(p, vec3(.3, 1.4, .4), .7));
    // Arm Left
    const vec3 arm_l_or = vec3(1.1, 1.4, 0.);
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

    return sd_union(Distance(d, material_man, idx), Distance(e, material_gun, idx));
}

Distance sd_swordman(vec3 p, float swordanim, uint material_man, uint material_sword, uint idx)
{
    // Head
    float d = sd_sphere(p - vec3(0., 3.1, 0.), 1.);
    // Body
    d = min(d, sd_round_box(p, vec3(.3, 1.4, .4), .7));
    // Arm Left
    const vec3 arm_l_or = vec3(1.1, 1.4, 0.);
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
    const vec3 leg_or = vec3(-.5, -2., .0);
    const vec3 leg_v = vec3(0., -1., 0.);
    vec3 leg_p = p;
    leg_p.x = abs(leg_p.x);
    d = min(d, sd_capsule(leg_p, leg_or, leg_or + leg_v * leg_length, .4));

    // Sword
    const vec3 p_sword = (p - arm_r_2_end) * rot_x(-swordanim - 0.4);
    const float holder_y = 2.8;
    float e = sd_box(p_sword - vec3(0., holder_y * .7, 0.), vec3(.1, holder_y, .2));
    e = min(e, sd_box(p_sword  - vec3(0., holder_y * .15, 0.), vec3(.5, .1, .4)));

    return sd_union(Distance(d, material_man, idx), Distance(e, material_sword, idx));
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

float noise1(sampler3D tex, in vec3 x)
{
    return textureLod(tex, (x + .5) / 32., 0.).x;
}

float noise1(sampler2D tex, in vec2 x)
{
    return textureLod(tex, (x + 0.5) / 64., 0.).x;
}

float fbm1(sampler2D tex, in vec2 x)
{
    float f = 0.0;
    f += 0.5000 * noise1(tex, x); x*=2.01;
    f += 0.2500 * noise1(tex, x); x*=2.01;
    f += 0.1250 * noise1(tex, x); x*=2.01;
    f += 0.0625 * noise1(tex, x);
    f = 2.0 * f - 0.9375;
    return f;
}

float sd_cone(in vec3 p, in vec2 c)
{
    vec2 q = vec2( length(p.xz), p.y );

    vec2 a = q - c*clamp( (q.x*c.x+q.y*c.y)/dot(c,c), 0.0, 1.0 );
    vec2 b = q - c*vec2( clamp( q.x/c.x, 0.0, 1.0 ), 1.0 );

    float s = -sign( c.y );
    vec2 d = min( vec2( dot( a, a ), s*(q.x*c.y-q.y*c.x) ),
    vec2( dot( b, b ), s*(q.y-c.y)  ));
    return -sqrt(d.x)*sign(d.y);
}

float sd_fake_round_cone(vec3 p, float b, float r1, float r2)
{
    float h = clamp( p.y/b, 0.0, 1.0 );
    p.y -= b*h;
    return length(p) - mix(r1,r2,h);
}

float sd_tree(in vec3 p)
{
    //float d = 999999999999.;
    float h = 123. * (p.x / 200.) + 17. * (p.z / 200.);
    float hei = 8. + 1. * sin(1.3 * h + 2);
    p.x = p.x + 0.5 * sin (1.2 * h);
    p.z = p.z + 0.5 * sin (1.7 * h);
    float d = sd_fake_round_cone(p, hei, .4, 0.);
    // vertical domain repetition
    // 3rd arg in clamp = number of repetition
    p.y *= -1.;
    p.y += hei;
    p.y -= clamp(floor(1.3 * p.y), 0., 9.) / 1.6;
    d = min(d, sd_cone(p, vec2(2.5, 2.)));
    return d;
}

float sd_baseground(vec3 p)
{
    float d1 = 0.05 - textureLod(sampler2D(pebbles_texture, common_sampler), p.xz * 0.1, 0.).x * .4;
    return p.y + d1;
}

Distance sd_ground(vec3 p)
{
    float grassheight = 0.1 - textureLod(sampler2D(pebbles_texture, common_sampler), p.xz * 4.0, 0.).x * .1;
    return Distance(sd_baseground(p) + grassheight, MATERIAL_GREEN, SENTINEL_IDX);
}

Distance scene_dist(vec3 pos)
{
    Distance m = Distance(sd_box(pos - vec3(0., -0.5, 0.), vec3(10., .5, 10.)), MATERIAL_CHECKER, SENTINEL_IDX);
//    m = sd_union(m, Distance(sd_tree(pos - vec3(2., 0., 2.)), MATERIAL_GREEN, SENTINEL_IDX));
    m = sd_union(m, sd_ground(pos - vec3(0., -0.5, 0.)));

    for (uint i = 0u; i < queuecount_raymarchmaxstep_aostep.x; i++) {
        vec3 pos_transformed = (vec4(pos - queue[i].position, 1.) * queue[i].rotation).xyz;
        switch (queue[i].shape_type_materials_id.x) {
            case SHAPE_TYPE_BOX:
                m = sd_union(m,
                    Distance(
                        sd_box(
                            pos_transformed,
                            queue[i].shape_data1.xyz),
                            queue[i].shape_type_materials_id.y,
                            i));
                break;
            case SHAPE_TYPE_SPHERE:
                m = sd_union(m,
                    Distance(
                        sd_sphere(
                            pos_transformed,
                            queue[i].shape_data1.x),
                            queue[i].shape_type_materials_id.y,
                            i));
                break;
            case SHAPE_TYPE_CYLINDER:
                m = sd_union(m,
                    Distance(
                        sd_cylinder(
                            pos_transformed,
                            queue[i].shape_data1.xyz,
                            queue[i].shape_data2.xyz,
                            queue[i].shape_data1.w),
                            queue[i].shape_type_materials_id.y,
                            i));
                break;
            case SHAPE_TYPE_GUNMAN:
                {
                    const float s = 0.2;
                    Distance d = sd_gunman(
                        (pos_transformed) / s * rot_y(queue[i].shape_data2.y),
                        queue[i].shape_data2.x,
                        queue[i].shape_type_materials_id.y,
                        queue[i].shape_type_materials_id.z,
                        i);
                    d.distance *= s;
                    m = sd_union(m, d);
                }
                break;
            case SHAPE_TYPE_SWORDMAN:
                {
                    const float s = 0.2;
                    Distance d = sd_swordman(
                        (pos_transformed) / s * rot_y(queue[i].shape_data2.y),
                        queue[i].shape_data2.x,
                        queue[i].shape_type_materials_id.y,
                        queue[i].shape_type_materials_id.z,
                        i);
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
    uint idx = 0;
    vec3 current_pos = vec3(0);

    for (uint i = 0u; i < queuecount_raymarchmaxstep_aostep.y; i++) {
        current_pos = ray_origin + d * ray_dir;
        Distance closest_distance = scene_dist(current_pos);

        if (abs(closest_distance.distance) < 0.0001 || d >= 100.0) {
            break;
        }

        d += closest_distance.distance;
        mat_id = closest_distance.materialId;
        idx = closest_distance.idx;
    }

    return Distance(d, mat_id, idx);
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
        {
            vec3 texture_xz = textureLod(sampler2D(checker_texture, common_sampler), ray_hit_pos.xz, 0.).rgb;
            vec3 texture_xy = textureLod(sampler2D(checker_texture, common_sampler), ray_hit_pos.xy, 0.).rgb;
            vec3 texture_yz = textureLod(sampler2D(checker_texture, common_sampler), ray_hit_pos.yz, 0.).rgb;
            col = texture_yz * abs(normal.x) + texture_xy * abs(normal.z) + texture_xz * abs(normal.y);
            break;
        }
        case MATERIAL_BLACK:
            col = vec3(0.);
            break;
        case MATERIAL_GREEN:
            col = vec3(0., 1., 0.);
            break;
        case MATERIAL_ORANGE:
            col = vec3(1.0, 0.30, 0.0);
            break;
        case MATERIAL_CRATE:
        {
            //vec3 pos_before_transformed = (vec4(ray_hit_pos + queue[d.idx].position, 1.) * inverse(queue[d.idx].rotation)).xyz;
            vec3 texture_xz = textureLod(sampler2D(crate_texture, common_sampler), ray_hit_pos.xz * .5 + .5, 0.).rgb;
            vec3 texture_xy = textureLod(sampler2D(crate_texture, common_sampler), ray_hit_pos.xy * .5 + vec2(.5, 0.), 0.).rgb;
            vec3 texture_yz = textureLod(sampler2D(crate_texture, common_sampler), ray_hit_pos.yz * .5 + vec2(0., .5), 0.).rgb;
            col = texture_yz * abs(normal.x) + texture_xy * abs(normal.z) + texture_xz * abs(normal.y);
            break;
        }
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
