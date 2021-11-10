#version 450

//#define DEV
#define EPS 0.0001
#define MAX_DISTANCE 100.0
#define MAX_QUEUE 100

#define SHAPE_TYPE_NONE 0
#define SHAPE_TYPE_SPHERE 1
#define SHAPE_TYPE_BOX 2
#define SHAPE_TYPE_GUN 3
#define SHAPE_TYPE_CAPSULE_LINE 4

struct RenderQueue
{
    vec3 position;
    vec3 scale;
    vec3 rotation;
    vec3 color;
    vec4 shape_data;
    vec4 shape_data2;
    uint shape_type;
    uint type;
};

layout(std140, binding = 0) uniform rendering_info {
    vec3 reso_time;
    vec3 cam_pos;
    vec3 cam_dir;
    float fov;
    uint queue_count;
};

#ifdef IS_WEB
layout(std140, binding = 1) uniform render_queue {
#else
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

float sd_silver_horn(vec3 pos, vec3 c)
{
    pos -= c;
    vec3 body_p = pos;

    float body = sd_round_box(
    body_p, vec3(1.60, 0.30, 0.10));

    float in_pl = dot(pos - vec3(1, 1.25, 0), normalize(vec3(-0.20, 1, 0))) + 1.5;

    body = max(body, -(dot(pos, normalize(vec3(1, 0.08, 0))) + 1.55));
    body = max(body, -(dot(pos, normalize(vec3(-1, -0.45, 0))) + 1.40));
    body = max(body, -in_pl);
    body = max(body, -(dot(vec3(pos.x, abs(pos.yz)) - vec3(0, 0.30, 0.10), normalize(vec3(0, -1, -1))) - 0.05));

    vec3 hp = pos - vec3(1.30, -.80, .0);
    hp = rot_z(radians(20)) * hp;

    float handle = sd_round_box(
    hp, vec3(0.30, 0.75, 0.10));

    handle = max(handle, -(dot(vec3(abs(hp.x), hp.y, abs(hp.z)) - vec3(0.30, 0.0, 0.10), normalize(vec3(-1, 0, -1))) - 0.04));

    float main_part = min(handle, body);
    //main_part = max(main_part, -(dot(vec3(pos.xy, abs(pos.z)), normalize(vec3(0, 0, -1))) + 0.10));

    vec3 tp = pos - vec3(0.60, -0.45, 0);
    float trigger_frame = sd_box(tp, vec3(0.50, 0.20, 0.05));

    trigger_frame = max(trigger_frame, -(dot(tp - vec3(-0.50, 0.20, 0.05), normalize(vec3(1, 0.2, 0)))));

    vec3 hole_p = pos - vec3(0.50, -0.46, 0);
    hole_p.xy = abs(hole_p.xy);

    float trigger_hole = length(hole_p.xy - vec2(clamp(hole_p.x, 0.0, 0.14), 0)) - 0.16;

    //pos.x -= pos.y * pos.y;
    //pos.z *= 1.5;

    return min(main_part, max(trigger_frame, -trigger_hole)); //length(pos.xz) - 0.2;
    //return length(pos.xz) - 0.2;cd
}

vec3 repeat(vec3 pos, vec3 c)
{
    return mod(pos + 0.5 * c, c) - 0.5 * c;
}

Distance scene_dist(vec3 pos)
{
    Distance m = Distance(sd_plane(pos, vec3(0, 1, 0), 0), 3);

    for (uint i = 0u; i < 100; i++) {
        switch (queue[i].shape_type) {
            case SHAPE_TYPE_SPHERE:
            m = sd_union(m, Distance(sd_sphere(pos - queue[i].position, queue[i].shape_data.x), 4));
            break;
            case SHAPE_TYPE_CAPSULE_LINE:
            m = sd_union(m,
                Distance(
                    sd_capsule_line(
                        pos - queue[i].position,
                        queue[i].shape_data.xyz,
                        queue[i].shape_data2.xyz,
                        queue[i].shape_data.w),
                        4));
            break;
            default:
            break;
        }
    }

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

    for (uint i = 0u; i < 100u; i++) {
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
    for (int i = 0; i < 2; i++)
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

    //vec2 uv = gl_FragCoord.xy / reso_time.xy * 2.0 - 1.0;
    vec3 ray_view_dir = rayViewDir(reso_time.xy, gl_FragCoord.xy);
    mat4 view_to_world = viewMatrix(cam_pos, cam_dir, world_up);

    vec3 ray_world_dir = (view_to_world * vec4(ray_view_dir, 0.0)).xyz;

    vec3 light_pos = vec3(0.0, 8.0, 0.0);
//    vec3 cam_up = vec3(0.0, 1.0, 0.0);
//    vec3 dir = lookat(uv, cam_pos, cam_dir, cam_up);
//    vec3 ray_dir = dir;

    Distance d = ray_march(cam_pos, ray_world_dir);

    if (d.distance > MAX_DISTANCE - EPS) {
        outColor = vec4(0.);
        return;
    }

    vec3 lp = cam_pos + d.distance * ray_world_dir;
    vec3 normal = get_normal(lp);
    float lv = 0.;

    lv += light(normal, lp, cam_pos) * 0.7;

    lv *= ambient_ocl(lp, normal);

    lv += 0.05;

    vec3 col = vec3(46., 209., 162.) / 255.;
    switch (d.materialId) {
        //case 0:
        //	col = vec3(0.);
        //	break;
        //case 1:
        //	col = vec3(192. / 255.);
        //	break;
//        case 2:
//        	col = vec3(46., 209., 162.) / 255.;
//        	break;
        case 3:
            col = vec3(0., 0., 1.);
            break;
        //	col = texture(uTextureGround, lp.xz * 0.5 + 0.5).rgb;
        //	//col = vec3(0.8, 0., 0.);
        //	break;
        case 4:
            col = vec3(1., 0., 0.);
            break;
        default:
            break;
    }

//    outColor = vec4(col, 1.0);
    outColor = vec4(clamp(pow(col * lv, 1. / vec3(2.2)), 0, 1), 1.0);
}