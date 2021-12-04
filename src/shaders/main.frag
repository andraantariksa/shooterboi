#version 450

#define EPS 0.0001
#define MAX_DISTANCE 100.0
#define MAX_QUEUE 100

#define SHAPE_TYPE_NONE 0
#define SHAPE_TYPE_BOX 1
#define SHAPE_TYPE_SPHERE 2
#define SHAPE_TYPE_CYLINDER 3
#define SHAPE_TYPE_GHOST 4

#define MATERIAL_GREEN 0
#define MATERIAL_YELLOW 1
#define MATERIAL_WHITE 2
#define MATERIAL_BLACK 3
#define MATERIAL_CHECKER 4
#define MATERIAL_RED 5
#define MATERIAL_ORANGE 6

struct RenderQueue
{
    vec3 position;
    vec3 scale;
    vec3 rotation;
    vec4 shape_data1;
    vec4 shape_data2;
    uvec2 shape_type_material_id;
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

#ifdef IS_WEB
layout(std140, binding = 2) uniform maze_data {
#else
layout(std430, binding = 2) readonly buffer maze_data {
#endif
    float maze[200][200];
};

layout(binding = 3) uniform sampler checker_sampler;
layout(binding = 4) uniform texture2D checker_texture;

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

Distance sd_union_loose(Distance d1, Distance d2, float f)
{
    return d1.distance < d2.distance * f ? d1 : d2;
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

float noise(vec2 pos)
{
    return abs(fract(sin(dot(pos ,vec2(19.9*pos.x,28.633*pos.y))) * 1341.9453*pos.x));
}

//const float slopeUB= 2.;
//const float g = sin(atan(1.,slopeUB));
float sd_maze(vec3 p) {
//    uint x_r = uint(round(p.x));
//    uint z_r = uint(round(p.z));

//    float x_a = maze[z_r][uint(floor(p.x))];
//    float x_b = maze[z_r][uint(ceil(p.x))];
//    float z_a = maze[uint(floor(p.z))][x_r];
//    float z_b = maze[uint(ceil(p.z))][x_r];
//    float h = mix(x_a, x_b, fract(p.x)) + mix(z_a, z_b, fract(p.z));

    float x_f = floor(p.x);
    float z_f = floor(p.z);
    float x_c = ceil(p.x);
    float z_c = ceil(p.z);
    float wx = (p.x - x_f) / (x_c - x_f);
    float wz = (p.z - z_f) / (z_c - z_f);

    uint x_f_ = uint(x_f);
    uint z_f_ = uint(z_f);
    uint x_c_ = uint(x_c);
    uint z_c_ = uint(z_c);

    float c0 = mix(maze[x_f_][z_f_], maze[x_c_][z_f_], wx);
    float c1 = mix(maze[x_f_][z_c_], maze[x_c_][z_c_], wx);

    float h = mix(c0, c1, wz);
//    return (p.y-h)*g;
    return (p.y-h)*3.0;
}

mat3 rot_z(float angle)
{
    float s = sin(angle);
    float c = cos(angle);
    return mat3(c, -s, 0, s, c, 0, 0, 0, 1);
}

mat3 rot_x(float angle)
{
    float s = sin(angle);
    float c = cos(angle);
    return mat3(1, 0, 0, 0, c, -s, 0, s, c);
}

mat2 rotmat(float angle)
{
    float c = cos(angle);
    float s = sin(angle);
    return mat2(c, s, -s, c);
}

float smin(float a, float b, float k)
{
    float h = clamp(0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
    return mix(b, a, h) - k * h * (1.0 - h);
}

float smax(float a,float b, float k)
{
    return -smin(-a,-b,k);
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

float sd_ghost(vec3 p)
{
    //float an=.5*.5*2.*6. +iMouse.x/iResolution.x*6.;
    //p.xz=mat2(cos(an),sin(an),sin(an),-cos(an))*p.xz;

    float time = reso_time.z;
    p *= rot_x(0.35);
    float time2=time*2.4;

    p.y+=2.3;
    p.xy*=rotmat(cos(time2+1.)*.04);
    p.y-=2.3;

    vec3 op=p;

    vec3 p2=p;
    p2.xy*=rotmat(cos(time2)*.1);

    vec3 p3=p;
    p3.xy*=rotmat(cos(time2-.0-length(p)/2.)*.13);

    float d=1e4;
    p.x=abs(p.x);
    p2.x=abs(p2.x);
    p3.x=abs(p3.x);

    d=smin(length(p2-vec3(-.75,0.,-.1))-.4,length(p2-vec3(.75,0.,-.1))-.5,2.);
    d=smin(d,length(p2-vec3(0,0.4,-.1))-.9,1.6);
    d+=.1;

    // Ears 1
    //d=smin(d,distance(vec3(.7,clamp(p3.y,0.,2.2),0.),p3.xyz)-.4,.14);
    //d=smax(d,-(length(p3-vec3(.7,1.7,-0.5))-.5),.2);

    // Neck
    d=smin(d,distance(vec3(0.,clamp(p.y,-1.6,-1.1),0.),p.xyz)-.6,.04);

    // Legs
    vec3 p4=op;
    float ld=-.75;
    p4.y-=ld;
    p4.yz*=rotmat(cos(time*2.+3.1415926/1.*0.)*.1);
    p4.y+=ld;
    p4.y-=max(0.,cos(time*2.+3.1415926/2.))*.1;
    d=smin(d,distance(vec3(.3,clamp(p4.y,-2.6,-2.),0.),p4.xyz)-.3,.1);
    p4=op;
    p4.y-=ld;
    p4.yz*=rotmat(cos(time*2.+3.1415926/1.*1.)*.1);
    p4.y+=ld;
    p4.y-=max(0.,cos(time*2.+3.1415926/2.+3.1415926/1.*1.))*.1;
    d=smin(d,distance(vec3(.3,clamp(p4.y,-2.6,-2.),0.),p4.xyz*vec3(-1,1,1))-.3,.1);

    // Belly
    d=smin(d,distance(vec3(0.,-1.5,-.2),p)-.5+cos(time*3.)*.03,.4);

    // Ears 2
    //d=smin(d,distance(vec3(1.1,2.3,-.1),p3)-.2,.8);

    // Tail
    //d=smin(d,distance(vec3(0,-1.7,.6),p)-.3,.1);

    vec3 q=vec3(0.35,.4,-1);

    /*
        if(mod(time-1.,4.)>.04)
        {
            d=smax(d,-(cylinder(p2-q,normalize(q-p2),.3,.1)-.0001),.05);
            d=smin(d,(length(p2-q*.9)-.2),.24);

            // Eye pupils
            if(op.x>0.)
                pupdist=(length(p2-vec3(.39,.32,-1.))-.2);
            else
                pupdist=(length(p2-vec3(.28,.32,-1.02))-.2);

            d=smin(d,pupdist,.005);
        }
    */

    // Nose
    d=smin(d,(length(p2-vec3(0,.1,-1.02))-.2),.02);

    // Mouth (two states)
    float d3=smax(-(length(p-vec3(-.05,-.29,-1.02))-.1),-(length(p-vec3(.05,-.29,-1.02))-.1),.1);

    float d2=max(p2.z,distance(p2,vec3(clamp(p2.x,0.,.3),-.2,clamp(p2.z,-2.,2.)))+.01);

    float time4=time/8.;
    float gg=smoothstep(0.,1.,clamp((min(fract(time4),1.-fract(time4))-.25)*64.,0.,1.));
    d=smax(d,mix(-d2,d3,gg),.1);

    // Tooth
    //d=min(d,(length(p-vec3(.0,-.2,-1.02))-.08));

    p.y+=.2;
    p.xy*=rotmat(.4+cos(time2*2.)*.02);

    // Arms
    float armd=smin(distance(vec3(.2,clamp(p.y,-1.8,-0.),0.),p.xyz)-.2,
    distance(p,vec3(0.2,-1.7,0))-.2,.2);

    d=smin(d,armd,.05);

    return d;
}

vec3 repeat(vec3 pos, vec3 c)
{
    return mod(pos + 0.5 * c, c) - 0.5 * c;
}

Distance scene_dist(vec3 pos)
{
    Distance m = Distance(sd_plane(pos, vec3(0., 1., 0.), 0), 4);
//    m = sd_union_loose(m,
//            Distance(
//                sd_maze(pos),
//                    1),
//            0.8);

    for (uint i = 0u; i < queuecount_raymarchmaxstep_aostep.x; i++) {
        switch (queue[i].shape_type_material_id.x) {
            case SHAPE_TYPE_BOX:
                m = sd_union(m,
                    Distance(
                        sd_box(
                            pos - queue[i].position,
                            queue[i].shape_data1.xyz),
                            queue[i].shape_type_material_id.y));
                break;
            case SHAPE_TYPE_SPHERE:
                m = sd_union(m,
                        Distance(
                            sd_sphere(
                                pos - queue[i].position,
                                queue[i].shape_data1.x),
                                queue[i].shape_type_material_id.y));
                break;
            case SHAPE_TYPE_GHOST:
                const float scale = 0.5;
                m = sd_union(m,
                    Distance(
                        sd_ghost(
                            (pos - queue[i].position) / scale) * scale,
                            queue[i].shape_type_material_id.y));
                            break;
            case SHAPE_TYPE_CYLINDER:
                m = sd_union(m,
                    Distance(
                        sd_cylinder(
                            pos - queue[i].position,
                            queue[i].shape_data1.xyz,
                            queue[i].shape_data2.xyz,
                            queue[i].shape_data1.w),
                            queue[i].shape_type_material_id.y));
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
    color *= ambient_ocl(ray_hit_pos, normal);
    outColor = vec4(color, 1.0);
}
