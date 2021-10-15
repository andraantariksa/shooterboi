#ifndef SHADERS_MAINPS
#define SHADERS_MAINPS
const char* MAINPS = "#version 450\n"\
"\n"\
"//#define DEV\n"\
"#define EPS 0.0001\n"\
"#define MAX_DISTANCE 100.0\n"\
"#define MAX_QUEUE 100\n"\
"\n"\
"#define SHAPE_TYPE_NONE 0\n"\
"#define SHAPE_TYPE_SPHERE 1\n"\
"#define SHAPE_TYPE_BOX 2\n"\
"#define SHAPE_TYPE_GUN 3\n"\
"\n"\
"#define SHAPE_OP_UNION 0\n"\
"#define SHAPE_OP_INTERSECT 1\n"\
"#define SHAPE_OP_SUBTRACT 2\n"\
"\n"\
"struct RenderQueue\n"\
"{\n"\
"    vec3 position;\n"\
"    vec3 scale;\n"\
"    vec3 rotation;\n"\
"    vec3 color;\n"\
"    vec4 shape_data;\n"\
"    uint type;\n"\
"    uint shape_type;\n"\
"    uint shape_op;\n"\
"};\n"\
"\n"\
"layout(std140, binding = 0) uniform rendering_info {\n"\
"    vec3 reso_time;\n"\
"    vec3 cam_pos;\n"\
"    vec3 cam_dir;\n"\
"    uint queue_count;\n"\
"};\n"\
"\n"\
"layout(std430, binding = 1) buffer render_queue {\n"\
"    RenderQueue queue[100];\n"\
"};\n"\
"\n"\
"//uniform sampler2D uTextureGround;\n"\
"//uniform vec2 uResolution;\n"\
"//uniform float uTime;\n"\
"//uniform vec3 uCameraPosition;\n"\
"//#ifdef DEV\n"\
"//vec3 uCameraDirection = vec3(0.0, 0.0, -1.0);\n"\
"//#else\n"\
"//uniform vec3 uCameraDirection;\n"\
"//#endif\n"\
"\n"\
"layout(location = 0) out vec4 outColor;\n"\
"\n"\
"struct RayHit\n"\
"{\n"\
"    vec3 pos;\n"\
"    float dist;\n"\
"};\n"\
"\n"\
"struct Distance\n"\
"{\n"\
"    float distance;\n"\
"    uint materialId;\n"\
"};\n"\
"\n"\
"Distance sd_union(Distance d1, Distance d2)\n"\
"{\n"\
"    return d1.distance < d2.distance ? d1 : d2;\n"\
"}\n"\
"\n"\
"Distance sd_intersect(Distance d1, Distance d2)\n"\
"{\n"\
"    return d1.distance > d2.distance ? d1 : d2;\n"\
"}\n"\
"\n"\
"float sd_plane(vec3 pos, vec3 n, float h)\n"\
"{\n"\
"    return dot(pos, n) + h;\n"\
"}\n"\
"\n"\
"float sd_sphere(vec3 pos, float rad)\n"\
"{\n"\
"    return length(pos) - rad;\n"\
"}\n"\
"\n"\
"float sd_round_box(vec3 p, vec3 b)\n"\
"{\n"\
"    vec3 q = abs(p) - b;\n"\
"    return length(max(q, 0.0)) + min(max(q.x, max(q.y, q.z)), 0.0);\n"\
"}\n"\
"\n"\
"float sd_box(vec3 pos, vec3 size)\n"\
"{\n"\
"    vec3 d = abs(pos) - size;\n"\
"    return max(d.x, max(d.y, d.z));\n"\
"}\n"\
"\n"\
"mat3 rot_z(float angle)\n"\
"{\n"\
"    float s = sin(angle);\n"\
"    float c = cos(angle);\n"\
"    return mat3(c, -s, 0, s, c, 0, 0, 0, 1);\n"\
"}\n"\
"\n"\
"float smin(float a, float b, float k)\n"\
"{\n"\
"    float h = clamp(0.5 + 0.5 * (b - a) / k, 0.0, 1.0);\n"\
"    return mix(b, a, h) - k * h * (1.0 - h);\n"\
"}\n"\
"\n"\
"float sd_silver_horn(vec3 pos, vec3 c)\n"\
"{\n"\
"    pos -= c;\n"\
"    vec3 body_p = pos;\n"\
"\n"\
"    float body = sd_round_box(\n"\
"        body_p, vec3(1.60, 0.30, 0.10));\n"\
"\n"\
"    float in_pl = dot(pos - vec3(1, 1.25, 0), normalize(vec3(-0.20, 1, 0))) + 1.5;\n"\
"\n"\
"    body = max(body, -(dot(pos, normalize(vec3(1, 0.08, 0))) + 1.55));\n"\
"    body = max(body, -(dot(pos, normalize(vec3(-1, -0.45, 0))) + 1.40));\n"\
"    body = max(body, -in_pl);\n"\
"    body = max(body, -(dot(vec3(pos.x, abs(pos.yz)) - vec3(0, 0.30, 0.10), normalize(vec3(0, -1, -1))) - 0.05));\n"\
"\n"\
"    vec3 hp = pos - vec3(1.30, -.80, .0);\n"\
"    hp = rot_z(radians(20)) * hp;\n"\
"\n"\
"    float handle = sd_round_box(\n"\
"        hp, vec3(0.30, 0.75, 0.10));\n"\
"\n"\
"    handle = max(handle, -(dot(vec3(abs(hp.x), hp.y, abs(hp.z)) - vec3(0.30, 0.0, 0.10), normalize(vec3(-1, 0, -1))) - 0.04));\n"\
"\n"\
"    float main_part = min(handle, body);\n"\
"    //main_part = max(main_part, -(dot(vec3(pos.xy, abs(pos.z)), normalize(vec3(0, 0, -1))) + 0.10));\n"\
"\n"\
"    vec3 tp = pos - vec3(0.60, -0.45, 0);\n"\
"    float trigger_frame = sd_box(tp, vec3(0.50, 0.20, 0.05));\n"\
"\n"\
"    trigger_frame = max(trigger_frame, -(dot(tp - vec3(-0.50, 0.20, 0.05), normalize(vec3(1, 0.2, 0)))));\n"\
"\n"\
"    vec3 hole_p = pos - vec3(0.50, -0.46, 0);\n"\
"    hole_p.xy = abs(hole_p.xy);\n"\
"\n"\
"    float trigger_hole = length(hole_p.xy - vec2(clamp(hole_p.x, 0.0, 0.14), 0)) - 0.16;\n"\
"\n"\
"    //pos.x -= pos.y * pos.y;\n"\
"    //pos.z *= 1.5;\n"\
"\n"\
"    return min(main_part, max(trigger_frame, -trigger_hole)); //length(pos.xz) - 0.2;\n"\
"    //return length(pos.xz) - 0.2;cd	\n"\
"}\n"\
"\n"\
"vec3 repeat(vec3 pos, vec3 c)\n"\
"{\n"\
"    return mod(pos + 0.5 * c, c) - 0.5 * c;\n"\
"}\n"\
"\n"\
"Distance scene_dist(vec3 pos)\n"\
"{\n"\
"    Distance sph = Distance(sd_sphere(pos - vec3(2., 0.5, -2.), 0.5), 2);\n"\
"    sph = sd_union(sph, Distance(sd_sphere(pos - vec3(-2., 0.5, 2.), 0.5), 2));\n"\
"    Distance m = sd_union(Distance(sd_silver_horn(pos, vec3(0, 3, 0)), 1), sph);\n"\
"\n"\
"    for (uint i = 0u; i < queue_count; i++) {\n"\
"        Distance d = m;\n"\
"\n"\
"        switch (queue[i].shape_type) {\n"\
"            case SHAPE_TYPE_SPHERE:\n"\
"                d = Distance(sd_sphere(pos - queue[i].position, queue[i].shape_data.x), 2);\n"\
"                break;\n"\
"            default:\n"\
"                continue;\n"\
"        }\n"\
"\n"\
"        switch (queue[i].shape_op) {\n"\
"            case SHAPE_OP_UNION:\n"\
"                m = sd_union(m, d);\n"\
"                break;\n"\
"            case SHAPE_OP_SUBTRACT:\n"\
"                d.distance = -d.distance;\n"\
"                m = sd_intersect(m, d);\n"\
"                break;\n"\
"            default:\n"\
"                continue;\n"\
"        }\n"\
"    }\n"\
"\n"\
"    return sd_union(m, Distance(sd_plane(pos, vec3(0, 1, 0), 0), 3));\n"\
"}\n"\
"\n"\
"vec3 get_normal(vec3 pos)\n"\
"{\n"\
"    Distance center_dist = scene_dist(pos);\n"\
"    Distance dfx = scene_dist(vec3(pos.x - EPS, pos.y, pos.z));\n"\
"    Distance dfy = scene_dist(vec3(pos.x, pos.y - EPS, pos.z));\n"\
"    Distance dfz = scene_dist(vec3(pos.x, pos.y, pos.z - EPS));\n"\
"    vec3 normal = (center_dist.distance - vec3(dfx.distance, dfy.distance, dfz.distance)) / EPS;\n"\
"\n"\
"    return normal;\n"\
"}\n"\
"\n"\
"Distance ray_march(vec3 ray_origin, vec3 ray_dir)\n"\
"{\n"\
"    Distance dist_traveled = Distance(0.0, 0);\n"\
"    vec3 current_pos = vec3(0);\n"\
"\n"\
"    for (uint i = 0u; i < 100u; i++) {\n"\
"        current_pos = ray_origin + dist_traveled.distance * ray_dir;\n"\
"        Distance closest_distance = scene_dist(current_pos);\n"\
"        \n"\
"        if (abs(closest_distance.distance) < EPS) {\n"\
"            break;\n"\
"        }\n"\
"\n"\
"        dist_traveled.distance += closest_distance.distance;\n"\
"        dist_traveled.materialId = closest_distance.materialId;\n"\
"\n"\
"        if (dist_traveled.distance >= MAX_DISTANCE) {\n"\
"            break;\n"\
"        }\n"\
"    }\n"\
"\n"\
"    return dist_traveled;\n"\
"}\n"\
"\n"\
"float soft_shadow(in vec3 ro, in vec3 rd, float mint, float maxt, float k)\n"\
"{\n"\
"    float res = 1.0;\n"\
"    float ph = 1e10;\n"\
"    for (float t = mint; t < maxt; )\n"\
"    {\n"\
"        Distance h = scene_dist(ro + rd * t);\n"\
"        if (h.distance < 0.001)\n"\
"            return 0.0;\n"\
"        float y = h.distance * h.distance / (2.0 * ph);\n"\
"        float d = sqrt(h.distance * h.distance - y * y);\n"\
"        res = min(res, k * d / max(0.0, t - y));\n"\
"        ph = h.distance;\n"\
"        t += h.distance;\n"\
"    }\n"\
"    return res;\n"\
"}\n"\
"\n"\
"float ambient_ocl(in vec3 pos, in vec3 nor)\n"\
"{\n"\
"    float occ = 0.0;\n"\
"    float sca = 1.0;\n"\
"    for (int i = 0; i < 5; i++)\n"\
"    {\n"\
"        float h = 0.001 + 0.15 * float(i) / 4.0;\n"\
"        Distance d = scene_dist(pos + h * nor);\n"\
"        occ += (h - d.distance) * sca;\n"\
"        sca *= 0.95;\n"\
"    }\n"\
"    return clamp(1.0 - 1.5 * occ, 0.0, 1.0);\n"\
"}\n"\
"\n"\
"float light(vec3 n, vec3 lp, vec3 l)\n"\
"{\n"\
"    return clamp(dot(n, normalize(l - lp)), 0, 1);\n"\
"}\n"\
"\n"\
"vec3 lookat(vec2 uv, vec3 pos, vec3 dir, vec3 up)\n"\
"{\n"\
"    vec3 c_dir = normalize(dir - pos);\n"\
"    vec3 right = normalize(cross(up, c_dir));\n"\
"    vec3 c_up = normalize(cross(c_dir, right));\n"\
"    return normalize(uv.x * right + uv.y * c_up + c_dir * 2.0);\n"\
"}\n"\
"\n"\
"void main()\n"\
"{\n"\
"    vec2 uv = gl_FragCoord.xy / reso_time.xy * 2.0 - 1.0;\n"\
"    uv.x *= reso_time.x / reso_time.y;\n"\
"\n"\
"    //uv *= 0.5;\n"\
"\n"\
"    //vec3 light_pos = vec3(sin(uTime) * 8, 8.0, -cos(uTime) * 8);\n"\
"    vec3 light_pos = vec3(0.0, 8.0, 0.0);\n"\
"    //vec3 cam_pos = uCameraPosition; // vec3(sin(uTime) * 8, 3.0, -cos(uTime) * 8);\n"\
"    //vec3 cam_pos = vec3(0, 10, 0.1);\n"\
"    //vec3 cam_dir = uCameraDirection;\n"\
"    vec3 cam_up = vec3(0.0, 1.0, 0.0);\n"\
"    vec3 dir = lookat(uv, cam_pos, cam_pos + cam_dir, cam_up);\n"\
"    //vec3 cam_dir = normalize(vec3(sin(uTime), 2.0, cos(uTime)) - cam_pos);\n"\
"    vec3 ray_dir = dir;\n"\
"\n"\
"    Distance d = ray_march(cam_pos, ray_dir);\n"\
"\n"\
"    if (d.distance > MAX_DISTANCE - EPS) {\n"\
"        outColor = vec4(0.);\n"\
"        return;\n"\
"    }\n"\
"\n"\
"    vec3 lp = cam_pos + d.distance * ray_dir;\n"\
"    vec3 normal = get_normal(lp);\n"\
"    float lv = 0.;\n"\
"\n"\
"    lv += light(normal, lp, light_pos) * 0.7;\n"\
"\n"\
"    lv *= ambient_ocl(lp, normal);\n"\
"\n"\
"    lv += 0.05;\n"\
"\n"\
"    vec3 col = vec3(46., 209., 162.) / 255.;\n"\
"    //switch (d.materialId) {\n"\
"    //case 0:\n"\
"    //	col = vec3(0.);\n"\
"    //	break;\n"\
"    //case 1:\n"\
"    //	col = vec3(192. / 255.);\n"\
"    //	break;\n"\
"    //case 2:\n"\
"    //	col = vec3(46., 209., 162.) / 255.;\n"\
"    //	break;\n"\
"    //case 3:\n"\
"    //	col = texture(uTextureGround, lp.xz).rgb;\n"\
"    //	//col = vec3(0.8, 0., 0.);\n"\
"    //	break;\n"\
"    //}\n"\
"\n"\
"    outColor = vec4(clamp(pow(col * lv, 1. / vec3(2.2)), 0, 1), 1.0);\n"\
"    //outColor = vec4(ray_dir, 1.0);\n"\
"}"; 
#endif