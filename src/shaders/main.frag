#version 450

#extension GL_GOOGLE_include_directive : enable

#include "common.glsl"

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
    return Distance(sd_baseground(p) + grassheight, MATERIAL_GRASS, SENTINEL_IDX);
}

#define TIME_FACTOR .01

#define SKY_COLOR vec3(.05, .4, 1.)
#define CLOUD_COLOR vec3(1, 1, 1)
#define CLOUD_DARK_COLOR vec3(.4, .5, .4)
#define LAYERS 3 // More layers = less fluffy
#define CLOUD_INTENSITY 1.4 //amount of clouds
#define DISTANCE .2 //scale factor of first layer
#define ROUGHNESS .75 //amount contribution per each successive layer

const float div = 1. / (float(LAYERS) * ROUGHNESS);

float fractal_cloud(vec2 position, vec2 delta)
{
    float step = DISTANCE;
    float mul = CLOUD_INTENSITY;
    float p = 0.;
    for (uint i = 0; i < LAYERS; i++)
    {
        vec2 coord = position * step + delta * sqrt(step);
        p = p + textureLod(sampler2D(abstract3_texture, common_sampler), coord, 0.).r * mul;
        step *= 1.75;
        mul *= ROUGHNESS;
    }
    return p * div;
}


#define MASK_LAYERS 2
float cloud_mask(vec2 position, vec2 delta)
{
    float step = .15;
    float p = .25;
    float mul = 1.;
    for (uint i = 0; i < MASK_LAYERS; i++)
    {
        vec2 coord = (position) * step + delta;// + sqrt(delta);
        float k = textureLod(sampler2D(pebbles_texture, common_sampler), coord, 0.).r;
        //k = cos(k * TAU);
        p += k * mul;
        mul = mul * -.5;
        step = step * 2.;
    }
    return p;
}

vec3 clouds(vec3 pos) {
    vec2 uv = pos.xz / 3000.;
    float time = reso_time.z * TIME_FACTOR;

    float cloudIntensity = sin(time * .5 + uv.x * 1.5);
    cloudIntensity = 1.25 + cloudIntensity * .75;

    float p = fractal_cloud(uv, vec2(time * .3, time * -.15));
    p *= (.5 + cloud_mask(uv, vec2(time * .15, time * .05)) * cloudIntensity);
    p = smoothstep(0., 2., p) * 2.;

    float dark = max(p - 1., 0.);
    dark =log(1. + dark);
    float light = min(p, 1.);
    return mix(
        mix(SKY_COLOR, CLOUD_COLOR, smoothstep(0.,1.,light)),
        CLOUD_DARK_COLOR, smoothstep(0.,1.,dark));
}

float hash1( vec2 p )
{
    p  = 50.0*fract( p*0.3183099 );
    return fract( p.x*p.y*(p.x+p.y) );
}

float noise( in vec2 x )
{
    vec2 p = floor(x);
    vec2 w = fract(x);
    #if 1
    vec2 u = w*w*w*(w*(w*6.0-15.0)+10.0);
    #else
    vec2 u = w*w*(3.0-2.0*w);
    #endif

    float a = hash1(p+vec2(0,0));
    float b = hash1(p+vec2(1,0));
    float c = hash1(p+vec2(0,1));
    float d = hash1(p+vec2(1,1));

    return -1.0+2.0*(a + (b-a)*u.x + (c-a)*u.y + (a - b - c + d)*u.x*u.y);
}

const mat2 m2 = mat2(  0.80,  0.60,
-0.60,  0.80 );

float fbm_9( in vec2 x )
{
    float f = 1.9;
    float s = 0.55;
    float a = 0.0;
    float b = 0.5;
    for(uint i = 0; i < 9; i++)
    {
        float n = noise(x);
        a += b*n;
        b *= s;
        x = f*m2*x;
    }

    return a;
}

float sd_terrain(vec2 p) {
    const float sca = 0.08;
    p *= sca;
    float h = fbm_9(p + vec2(1.0,-2.0) );
    return h * 1.8;
}

Distance scene_dist(vec3 pos) {
    Distance m = Distance(pos.y - sd_terrain(pos.xz), MATERIAL_GRASS, SENTINEL_IDX);
    //m = sd_union(m, Distance(sd_box(pos - vec3(0., -0.5, 0.), vec3(10., .5, 10.)), MATERIAL_COBBLESTONE_PAVING, SENTINEL_IDX));
    //m = sd_union(m, sd_ground(pos - vec3(0., -0.5, 0.)));

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
                return m;
        }
    }

    return m;
}

const float SKY_HEIGHT = 500.;
const vec3 SKYCOLOR = vec3(113, 188, 225) / 255.0;

vec3 texture_map_triplanar(texture2D texture, vec3 pos, vec3 normal) {
    vec3 texture_xz = textureLod(sampler2D(texture, common_sampler), pos.xz * .5 + .5, 0.).rgb;
    vec3 texture_xy = textureLod(sampler2D(texture, common_sampler), pos.xy * .5 + vec2(.5, 0.), 0.).rgb;
    vec3 texture_yz = textureLod(sampler2D(texture, common_sampler), pos.yz * .5 + vec2(0., .5), 0.).rgb;
    return texture_yz * abs(normal.x) + texture_xy * abs(normal.z) + texture_xz * abs(normal.y);
}

vec3 texture_map_triplanar(texture2D texture, vec3 pos, vec3 normal, vec4 xz_tex, vec4 xz_world, vec4 xy_tex, vec4 xy_world, vec4 yz_tex, vec4 yz_world) {
    vec3 texture_xz = textureLod(sampler2D(texture, common_sampler), mix(xz_tex.xy, xz_tex.zw, (pos.xz - xz_world.xy) / (xz_world.zw - xz_world.xy)), 0.).rgb;
    vec3 texture_xy = textureLod(sampler2D(texture, common_sampler), mix(xy_tex.xy, xy_tex.zw, (pos.xy - xy_world.xy) / (xy_world.zw - xy_world.xy)), 0.).rgb;
    vec3 texture_yz = textureLod(sampler2D(texture, common_sampler), mix(yz_tex.xy, yz_tex.zw, (pos.yz - yz_world.xy) / (yz_world.zw - yz_world.xy)), 0.).rgb;
    return texture_yz * abs(normal.x) + texture_xy * abs(normal.z) + texture_xz * abs(normal.y);
}

void main()
{
    vec3 ray_view_dir = ray_view_dir(reso_time.xy, gl_FragCoord.xy);
    mat4 view_to_world = view_matrix(cam_pos, cam_dir);
    vec3 ray_world_dir = (view_to_world * vec4(ray_view_dir, 0.0)).xyz;

    Distance d = ray_march(cam_pos, ray_world_dir);

    if (d.distance > MAX_DISTANCE - EPS) {
        if (dot(ray_world_dir, WORLD_UP) > 0.) {
            float h_f = SKY_HEIGHT / ray_world_dir.y;
            vec3 ray_hit_sky_pos = cam_pos + h_f * ray_world_dir;
            outColor = vec4(clouds(ray_hit_sky_pos), 1.);
            return;
        } else {
            discard;
        }
    }

    vec3 ray_hit_pos = cam_pos + d.distance * ray_world_dir;
    vec3 normal = get_normal(ray_hit_pos);

    vec3 col = vec3(0.);
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
            //col = texture_map_triplanar(crate_texture, ray_hit_pos, normal);

            // Assuming SHAPE_TYPE_BOX
            // Top back right
            const vec3 tbr = queue[d.idx].position + queue[d.idx].shape_data1.xyz;
            // Bottom front left
            const vec3 bfl = queue[d.idx].position - queue[d.idx].shape_data1.xyz;
            col = texture_map_triplanar(
                crate_texture,
                ray_hit_pos,
                normal,
                vec4(0., 0., 1., 1.),
                vec4(bfl.x, bfl.z, tbr.x, tbr.z),
                vec4(0., 0., 1., 1.),
                vec4(bfl.x, bfl.y, tbr.x, tbr.y),
                vec4(0., 0., 1., 1.),
                vec4(bfl.y, bfl.z, tbr.y, tbr.z));
            break;
        }
        case MATERIAL_GRASS:
        {
            //vec3 pos_before_transformed = (vec4(ray_hit_pos + queue[d.idx].position, 1.) * inverse(queue[d.idx].rotation)).xyz;
            col = texture_map_triplanar(grass_texture, ray_hit_pos, normal);
            break;
        }
        case MATERIAL_COBBLESTONE_PAVING:
        {
            col = texture_map_triplanar(cobblestone_paving_texture, ray_hit_pos, normal);
            break;
        }
        case MATERIAL_STONE_WALL:
        {
            col = texture_map_triplanar(stone_wall_texture, ray_hit_pos, normal);
            break;
        }
        case MATERIAL_CONTAINER:
        {
            // xz yz xy
            // w 322
            // h 419.5

            // Assuming SHAPE_TYPE_BOX
            // Top back right
            const vec3 tbr = queue[d.idx].position + queue[d.idx].shape_data1.xyz;
            // Bottom front left
            const vec3 bfl = queue[d.idx].position - queue[d.idx].shape_data1.xyz;
            col = texture_map_triplanar(
                container_texture,
                ray_hit_pos,
                normal,
                vec4(0., 0., 1., .5),
                vec4(bfl.x, bfl.z, tbr.x, tbr.z),
                vec4(0., .5, 322. / 840., 1.),
                vec4(bfl.x, bfl.y, tbr.x, tbr.y),
                vec4(0., 0., 1., .5),
                vec4(bfl.y, bfl.z, tbr.y, tbr.z));
            break;
        }
        case MATERIAL_TARGET:
        {
            float dd = dot(normal, normalize(queue[d.idx].position - cam_pos)) * -1. / 3.;
            col = textureLod(sampler2D(target_texture, common_sampler), vec2(1. - dd), 0.).rgb;
            break;
        }
        default:
            break;
    }

    const vec3 ambient_light = SKYCOLOR * col * 0.5;
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
