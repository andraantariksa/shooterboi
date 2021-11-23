#version 450

#define EPS 0.0001
#define MAX_DISTANCE 100.0

struct RenderQueue
{
    vec3 position;
    vec3 scale;
    vec3 rotation;
    vec4 shape_data;
    vec4 shape_data2;
    uint shape_type;
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
// std430
layout(std430, binding = 1) readonly buffer render_queue {
#endif
    RenderQueue queue[50];
};

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

float sd_capsule_line(vec3 p, vec3 a, vec3 b, float r)
{
    vec3 pa = p - a, ba = b - a;
    float h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h) - r;
}

float sd_terrain(vec3 pos)
{

    return 0.0;
}

float sd_plane(vec3 pos, vec3 n, float h)
{
    return dot(pos, n) + h;
}

float sd_sphere(vec3 pos, float rad)
{
    return length(pos) - rad;
}

float sd_round_box(vec3 p, vec3 b)
{
    vec3 q = abs(p) - b;
    return length(max(q, 0.0)) + min(max(q.x, max(q.y, q.z)), 0.0);
}

float sd_box(vec3 pos, vec3 size)
{
    vec3 d = abs(pos) - size;
    return max(d.x, max(d.y, d.z));
}

mat3 rot_z(float angle)
{
    float s = sin(angle);
    float c = cos(angle);
    return mat3(c, -s, 0, s, c, 0, 0, 0, 1);
}

float smin(float a, float b, float k)
{
    float h = clamp(0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
    return mix(b, a, h) - k * h * (1.0 - h);
}

mat3 rot_y(float rad) {
    float c = cos(rad);
    float s = sin(rad);
    return mat3(
    c, 0.0, -s,
    0.0, 1.0, 0.0,
    s, 0.0, c
    );
}

float sd_holding_hand(vec3 pos, vec3 c)
{
    pos -= c;
    pos *= rot_y(radians(-90.));

    vec3 base_p = pos - vec3(0, 0, 0.35);
    base_p = rot_y(radians(10)) * base_p;

    float base = sd_box(base_p, vec3(0.70, 0.70, 0.15));

    base = max(base, -(dot(vec3(base_p.x, abs(base_p.y), base_p.z) - vec3(0, .40, 0), normalize(vec3(-0.15, -1, 0))) + 0.2));
    base_p -= vec3(.05, .2, 0);
    base = max(base, -(dot(vec3(base_p.x, abs(base_p.y), base_p.z) - vec3(-.65, 0, 0), normalize(vec3(1, -0.2, 0)))));

    vec3 index_p = pos - vec3(-0.5, 0.5, .46);
    index_p = rot_y(radians(-10)) * index_p;
    index_p -= vec3(-.3, 0, 0);
    float finger = sd_box(index_p, vec3(.3, .1, .1));
    index_p = rot_y(radians(-0)) * (index_p - vec3(-.3, 0, 0));
    index_p -= vec3(-.2, 0, 0);
    finger = min(finger, sd_box(index_p, vec3(.2, .1, .1)));
    index_p = rot_y(radians(-0)) * (index_p - vec3(-.2, 0, 0));
    index_p -= vec3(-.15, 0, 0);
    finger = min(finger, sd_box(index_p, vec3(.15, .1, .1)));

    vec3 mid_p = pos - vec3(-0.6, 0.15, .48);
    mid_p = rot_y(radians(-40)) * mid_p;
    mid_p -= vec3(-.35, 0, 0);
    finger = min(finger, sd_box(mid_p, vec3(.35, .1, .1)));
    mid_p = rot_y(radians(-90)) * (mid_p - vec3(-.35, 0, 0));
    mid_p -= vec3(-.2, 0, 0);
    finger = min(finger, sd_box(mid_p, vec3(.2, .1, .1)));
    mid_p = rot_y(radians(-50)) * (mid_p - vec3(-.2, 0, 0));
    mid_p -= vec3(-.15, 0, 0);
    finger = min(finger, sd_box(mid_p, vec3(.15, .1, .1)));

    vec3 ring_p = pos - vec3(-0.6, -.15, .48);
    ring_p = rot_y(radians(-40)) * ring_p;
    ring_p -= vec3(-.3, 0, 0);
    finger = min(finger, sd_box(ring_p, vec3(.3, .1, .1)));
    ring_p = rot_y(radians(-90)) * (ring_p - vec3(-.3, 0, 0));
    ring_p -= vec3(-.2, 0, 0);
    finger = min(finger, sd_box(ring_p, vec3(.2, .1, .1)));
    ring_p = rot_y(radians(-40)) * (ring_p - vec3(-.2, 0, 0));
    ring_p -= vec3(-.15, 0, 0);
    finger = min(finger, sd_box(ring_p, vec3(.15, .1, .1)));

    vec3 little_p = pos - vec3(-0.50, -.45, .48);
    little_p = rot_y(radians(-40)) * little_p;
    little_p -= vec3(-.25, 0, 0);
    finger = min(finger, sd_box(little_p, vec3(.25, .1, .1)));
    little_p = rot_y(radians(-60)) * (little_p - vec3(-.25, 0, 0));
    little_p -= vec3(-.20, 0, 0);
    finger = min(finger, sd_box(little_p, vec3(.20, .1, .1)));
    little_p = rot_y(radians(-80)) * (little_p - vec3(-.2, 0, 0));
    little_p -= vec3(-.15, 0, 0);
    finger = min(finger, sd_box(little_p, vec3(.15, .1, .1)));

    //float index = sd_box(index_p, vec3(0.25, .13, .13));

    return min(base, finger);
}

float sd_silver_horn(vec3 pos, vec3 c)
{
    pos -= c;
    pos *= rot_y(radians(-90.));
    vec3 body_p = pos;

    float body = sd_round_box(
    body_p, vec3(1.60, 0.30, 0.125));

    float in_pl = dot(pos - vec3(0.9, 1.25, 0), normalize(vec3(-0.20, 1, 0))) + 1.5;

    body = max(body, -(dot(pos, normalize(vec3(1, 0.08, 0))) + 1.55));
    body = max(body, -(dot(pos, normalize(vec3(-1, -0.45, 0))) + 1.40));
    body = max(body, -in_pl);
    body = max(body, -(dot(vec3(pos.x, abs(pos.yz)) - vec3(0, 0.30, 0.125), normalize(vec3(0,-1,-1))) - 0.05));

    vec3 hp = pos - vec3(1.30, -.80, .0);
    hp = rot_z(radians(20)) * hp;

    float handle =  sd_round_box(
    hp, vec3(0.30, 0.75, 0.125));

    handle = max(handle, -(dot(vec3(abs(hp.x), hp.y, abs(hp.z)) - vec3(0.30, 0.0, 0.125), normalize(vec3(-1, 0, -1))) - 0.04));

    float main_part = min(handle, body);
    //main_part = max(main_part, -(dot(vec3(pos.xy, abs(pos.z)), normalize(vec3(0, 0, -1))) + 0.10));

    vec3 tp = pos - vec3(0.60, -0.45, 0);
    float trigger_frame = sd_box(tp, vec3(0.50, 0.20, 0.08));

    trigger_frame = max(trigger_frame, -(dot(tp - vec3(-0.50, 0.20, 0.05), normalize(vec3(1, 0.2, 0)))));

    vec3 hole_p = pos - vec3(0.50, -0.46, 0);
    float trigger_sw = sd_box(hole_p, vec3(.16, .16, .08));
    hole_p.xy = abs(hole_p.xy);

    float trigger_hole = length(hole_p.xy - vec2(clamp(hole_p.x, 0.0, 0.14), 0)) - 0.16;
    trigger_frame = max(trigger_frame, -trigger_hole);

    //pos.x -= pos.y * pos.y;
    //pos.z *= 1.5;

    return min(main_part, trigger_frame); //length(pos.xz) - 0.2;
    //return length(pos.xz) - 0.2;
}

vec3 repeat(vec3 pos, vec3 c)
{
    return mod(pos + 0.5 * c, c) - 0.5 * c;
}

mat3 rot_x(float rad)
{
    float c = cos(rad);
    float s = sin(rad);
    return mat3(1., 0., 0.,
               0., c, -s,
               0., s, c);
}

Distance scene_dist(vec3 pos)
{
    vec3 gun_pos = vec3(1., -0.5, -5.);
    mat3 rot = rot_x(fov_shootanim.x);
    Distance m = Distance(sd_silver_horn(pos * rot, gun_pos), 0);
    m = sd_union(m, Distance(sd_holding_hand(pos * rot, gun_pos + vec3(0.9, -1., 1.7)), 1));
    return m;
}

vec3 get_normal(vec3 pos)
{
    Distance center_dist = scene_dist(pos);
    Distance dfx = scene_dist(vec3(pos.x - EPS, pos.y, pos.z));
    Distance dfy = scene_dist(vec3(pos.x, pos.y - EPS, pos.z));
    Distance dfz = scene_dist(vec3(pos.x, pos.y, pos.z - EPS));
    vec3 normal = (center_dist.distance - vec3(dfx.distance, dfy.distance, dfz.distance)) / EPS;

    return normal;
}

Distance ray_march(vec3 ray_origin, vec3 ray_dir)
{
    Distance dist_traveled = Distance(0.0, 0);
    vec3 current_pos = vec3(0);

    for (uint i = 0u; i < queuecount_raymarchmaxstep_aostep.y; i++) {
        current_pos = ray_origin + dist_traveled.distance * ray_dir;
        Distance closest_distance = scene_dist(current_pos);

        if (abs(closest_distance.distance) < EPS) {
            break;
        }

        dist_traveled.distance += closest_distance.distance;
        dist_traveled.materialId = closest_distance.materialId;

        if (dist_traveled.distance >= MAX_DISTANCE) {
            break;
        }
    }

    return dist_traveled;
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

vec3 rayViewDir(vec2 size, vec2 coord) {
    vec2 xy = coord - size / 2.0;
    float z = size.y / tan(radians(60.) / 2.0);
    return normalize(vec3(xy, z));
}

mat4 viewMatrix(vec3 pos, vec3 dir, vec3 world_up) {
    vec3 right = normalize(cross(dir, world_up));
    vec3 up = normalize(cross(dir, right));
    return mat4(
        vec4(right, 0.0),
        vec4(up, 0.0),
        vec4(dir, 0.0),
        vec4(0.0, 0.0, 0.0, 1));
}

void main()
{
    vec3 world_up = vec3(0.0, 1.0, 0.0);
    vec3 cam_pos_ = vec3(0.);
    vec3 cam_dir_ = vec3(0., 0., -1.);

    vec3 ray_view_dir = rayViewDir(reso_time.xy, gl_FragCoord.xy);
    mat4 view_to_world = viewMatrix(cam_pos_, cam_dir_, world_up);

    vec3 ray_world_dir = (view_to_world * vec4(ray_view_dir, 0.0)).xyz;

    vec3 light_pos = vec3(0.0, 0.0, 0.0);

    Distance d = ray_march(cam_pos_, ray_world_dir);

    if (d.distance > MAX_DISTANCE - EPS) {
        discard;
    }

    vec3 lp = cam_pos_ + d.distance * ray_world_dir;
    vec3 normal = get_normal(lp);
    float lv = 0.;

    lv += light(normal, lp, cam_pos_) * 0.7;

    lv *= ambient_ocl(lp, normal);

    lv += 0.05;

    vec3 col = vec3(46., 209., 162.) / 255.;
    switch (d.materialId) {
        case 0:
            col = vec3(192. / 255.);
            break;
        case 1:
            col = vec3(232., 190., 172.) / 255.;
            break;
        default:
            break;
    }

    outColor = vec4(clamp(pow(col * lv, 1. / vec3(2.2)), 0, 1), 1.0);
}
