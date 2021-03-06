#version 450

#extension GL_GOOGLE_include_directive : enable

#include "common.glsl"
#include "hash.glsl"
#include "sdf.glsl"

#define SCENE_NONE 0
#define SCENE_FOREST 1
#define SCENE_CITY 2
#define SCENE_SNOW 3

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

    return sd_union(Distance(d, material_man, idx), Distance(e, material_sword, idx));
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
    for(uint i = 0; i < 3; i++)
    {
        float n = noise(x);
        a += b*n;
        b *= s;
        x = f*m2*x;
    }

    return a;
}

float sd_terrain(vec2 p) {
    //const float sca = 0.1;
    //float h = fbm_9(p * sca);
    float h = textureLod(sampler2D(terrain_texture, common_sampler), p * 0.05, 0.).r;
    return h * 2.5;
}

const float building_c = 30.;
const float building_c_h = building_c * .5;
Distance sd_city(vec3 p) {
    float d = 1e10;

    for (int i = -1; i <= 1; ++i)
    for (int j = -1; j <= 1; ++j)
    {
        vec2 o = vec2(i * building_c, j * building_c);
        vec2 r = p.xz + o;
        vec2 block = floor((r + building_c_h) / building_c);
        float height = hash1(block) * 20. + 10.;
        float x_s = mix(7., 14., hash1(block, height));
        float z_s = mix(7., 14., hash1(block, height + 2.));

        // Building base
        vec3 q = p; vec3(r.x, p.y, r.y);
        q.xz = mod(q.xz + building_c_h, building_c) - building_c_h;
        q.y -= height;
        q -= vec3(o.x, 0., o.y);
        d = min(d, sd_box(q, vec3(x_s, height, z_s)));

        q.y -= height;

        // Building 2nd floor
        for (uint k = 0; k < 3; ++k) {
            float x_s_2 = mix(0., x_s, hash1(block, height));
            float z_s_2 = mix(0., z_s, hash1(block, height + 2.));
            d = min(d, sd_box(q, vec3(5.)));
        }
    }
    Distance m = Distance(d * .9, MATERIAL_BUILDING, SENTINEL_IDX);
    return m;
}

const float tree_c = 10.;
const float tree_c_h = tree_c * .5;
Distance sd_trees(vec3 p, uint trunk_material, uint greenery_material) {
    Distance d = Distance(1e10, greenery_material, SENTINEL_IDX);

    for (int i = -1; i <= 1; ++i)
    for (int j = -1; j <= 1; ++j) {
        vec2 o = vec2(i * tree_c, j * tree_c);
        vec2 r = p.xz + o;
        vec2 block = floor((r + tree_c_h) / tree_c);
        float block_hash = hash1(block);
        float height = (block_hash * 9. + 6.) * (step(3., abs(block.x)) + step(5., abs(block.y)));

        vec3 q = p;
        q.xz = mod(q.xz + tree_c_h, tree_c) - tree_c_h;
        q -= vec3(o.x, 0., o.y);
        q.x += 2. * sin(1.2 * height + 0.);
        q.z += 1. * sin(1.7 * height + 1.);

        d = sd_union(
            d,
            Distance(
                sd_fake_round_cone(
                    q,
                    height,
                    .5,
                    .1),
                trunk_material,
                SENTINEL_IDX));
        uint greeneries = uint(height / 1.3);
        float d_greenery = 1e10;
        for (uint i = 0; i < greeneries; ++i) {
            d_greenery = min(
                    d_greenery,
                    sd_cone(
                        q - vec3(0., height + .3 - float(i) * .6, 0.),
                        vec2(2. + float(i) * .4, 3.),
                        1.));
        }
        d = sd_union(
                d,
                Distance(
                    d_greenery,
                    greenery_material,
                    SENTINEL_IDX));
    }
    return d;
}

Distance scene_dist(vec3 pos) {
    Distance m;

    switch (queuecount_raymarchmaxstep_aostep_background_type.w) {
        case SCENE_FOREST:
        {
            m = Distance(pos.y - sd_terrain(pos.xz), MATERIAL_GRASS, SENTINEL_IDX);
            m = sd_union(m, sd_trees(pos - vec3(0., -1.8, 0.), MATERIAL_TREE_BARK, MATERIAL_LEAVES));
            break;
        }
        case SCENE_CITY:
        {
            m = sd_city(pos);
            break;
        }
        case SCENE_SNOW:
        {
            m = Distance(pos.y + sd_terrain(pos.xz), MATERIAL_GRASS, SENTINEL_IDX);
            break;
        }
        default:
            break;
    }

    for (uint i = 0u; i < queuecount_raymarchmaxstep_aostep_background_type.x; i++) {
        vec3 pos_transformed = (queue[i].rotation * vec4(pos - queue[i].position_scale.xyz, 1.)).xyz;
        pos_transformed /= queue[i].position_scale.w;

        Distance d;
        switch (queue[i].shape_type_materials_id.x) {
            case SHAPE_TYPE_BOX:
                d = Distance(
                        sd_box(
                            pos_transformed,
                            queue[i].shape_data1.xyz),
                            queue[i].shape_type_materials_id.y,
                            i);
                break;
            case SHAPE_TYPE_SPHERE:
            {
                d = Distance(
                    sd_sphere(
                        pos_transformed,
                        queue[i].shape_data1.x),
                        queue[i].shape_type_materials_id.y,
                        i);
                break;
            }
            case SHAPE_TYPE_CYLINDER:
            {
                d = Distance(
                    sd_cylinder(
                        pos_transformed,
                        queue[i].shape_data1.xyz,
                        queue[i].shape_data2.xyz,
                        queue[i].shape_data1.w),
                        queue[i].shape_type_materials_id.y,
                        i);
                break;
            }
            case SHAPE_TYPE_GUNMAN:
            {
                d = sd_gunman(
                        pos_transformed,
                        queue[i].shape_data1.x,
                        queue[i].shape_type_materials_id.y,
                        queue[i].shape_type_materials_id.z,
                        i);
                break;
            }
            case SHAPE_TYPE_SWORDMAN:
            {
                d = sd_swordman(
                        pos_transformed,
                        queue[i].shape_data1.x,
                        queue[i].shape_type_materials_id.y,
                        queue[i].shape_type_materials_id.z,
                        i);
                break;
            }
            default:
                return m;
        }
        d.distance *= queue[i].position_scale.w;
        m = sd_union(m, d);
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

const vec3 STREET_COLOR = vec3(.4, .8, 1.0);
const vec3 windowColorA = vec3(0.0, 0.0, 1.5);
const vec3 windowColorB = vec3(0.5, 1.5, 2.0);
vec3 building_material(vec3 p, vec3 n) {
    vec2 block = floor((p.xz + building_c_h) / building_c);

    //float windowSize = 0.03 + 0.12 * hash1(block + 0.1);
    float windowProb = .1 * 0.3 + 0.8 * hash1(block + 0.2);
    float level = floor(p.y);
    vec3 colorA = mix(windowColorA, windowColorB, hash3(block));
    vec3 colorB = mix(windowColorA, windowColorB, hash3(block + 0.1));
    vec3 color = mix(colorA, colorB, hash1(block, level));
    color *= 0.3 + 0.6 * smoothstep(0.1, 0.5, noise(5.0 * p.xz + 100.0 * hash1(level)));
    color *= smoothstep(windowProb - 0.4, windowProb + 0.4, hash1(block, level + 0.1));

    vec3 street_color = STREET_COLOR * exp(p.y);
    color = mix(color, street_color, 0.);

    return color * step(n.y, EPS);
}

const vec3 fog_color = vec3(0.34, 0.37, 0.4);
vec3 apply_fog(vec3 color, float dist) {
    float dp = dist / MAX_DISTANCE;
    return mix(color, SKYCOLOR, smoothstep(0., 1., dp));
}

const vec3 street_color = vec3(0.890, 0.937, 0.949);
vec3 apply_street_color(vec3 color, float y) {
    const float startDist = 10.;
    float street_light_amount = exp(-(y-8.0) * (1.0/startDist));
    return mix(color, street_color, street_light_amount);
}

vec3 get_night_sky(vec3 dir) {
    float rand = textureLod(sampler2D(rgba_noise_medium, noise_sampler), dir.xz*.2, 0.).r;
    vec3 stars = pow(textureLod(sampler2D(gray_noise_small_texture, noise_sampler), 6.*dir.xz + .5, 0.).xxx,vec3(60.))*rand;
    vec3 moon  = vec3(2.) * textureLod(sampler2D(gray_noise_small_texture, noise_sampler), dir.xy*.5, 0.).xxx
        * (1. - step(max(pow(dot(dir,normalize(vec3(25.,25.,55.))),360.),0.), .7));
    return mix(fog_color, stars, smoothstep(-5., 1., dir.y)) + moon;
    //return vec3(rand);
}

vec3 color_mapping(vec3 ray_hit_pos, vec3 normal, Distance d)
{
    vec3 col;
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
//        case MATERIAL_CHECKER:
//        {
//            vec3 texture_xz = textureLod(sampler2D(checker_texture, common_sampler), ray_hit_pos.xz, 0.).rgb;
//            vec3 texture_xy = textureLod(sampler2D(checker_texture, common_sampler), ray_hit_pos.xy, 0.).rgb;
//            vec3 texture_yz = textureLod(sampler2D(checker_texture, common_sampler), ray_hit_pos.yz, 0.).rgb;
//            col = texture_yz * abs(normal.x) + texture_xy * abs(normal.z) + texture_xz * abs(normal.y);
//            break;
//        }
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
            // Assuming SHAPE_TYPE_BOX
            // Top back right
            const vec3 tbr = queue[d.idx].position_scale.xyz + queue[d.idx].shape_data1.xyz;
            // Bottom front left
            const vec3 bfl = queue[d.idx].position_scale.xyz - queue[d.idx].shape_data1.xyz;
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
            col = texture_map_triplanar(grass_texture, ray_hit_pos / 4.0, normal);
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
            const vec3 tbr = queue[d.idx].position_scale.xyz + queue[d.idx].shape_data1.xyz;
            // Bottom front left
            const vec3 bfl = queue[d.idx].position_scale.xyz - queue[d.idx].shape_data1.xyz;
            col = texture_map_triplanar(
            container_texture,
            ray_hit_pos,
            normal,
            vec4(0., 0., 1., .5),
            vec4(bfl.x, bfl.z, tbr.x, tbr.z),
            vec4(0., 1., 322. / 840., .5),
            vec4(bfl.x, bfl.y, tbr.x, tbr.y),
            vec4(0., 0., 1., .5),
            vec4(bfl.y, bfl.z, tbr.y, tbr.z));
            break;
        }
        case MATERIAL_TARGET:
        {
            float dd = dot(normal, normalize(queue[d.idx].position_scale.xyz - cam_pos)) * -1. / 3.;
            col = textureLod(sampler2D(target_texture, common_sampler), vec2(1. - dd), 0.).rgb;
            break;
        }
        case MATERIAL_TARGET_DIMMED:
        {
            float dd = dot(normal, normalize(queue[d.idx].position_scale.xyz - cam_pos)) * -1. / 3.;
            col = textureLod(sampler2D(target_texture, common_sampler), vec2(1. - dd), 0.).rgb;
            col /= 3.;
            break;
        }
        case MATERIAL_BUILDING:
        {
            col = building_material(ray_hit_pos, normal);
            break;
        }
        case MATERIAL_TREE_BARK:
        {
            vec2 uv = vec2(atan(normal.x, normal.z) / PI, ray_hit_pos.y * 3.)+.5;
            col = textureLod(sampler2D(tree_bark_texture, common_sampler), uv, 0.).rgb;
            break;
        }
        case MATERIAL_LEAVES:
        {
            col = texture_map_triplanar(leaves_texture, ray_hit_pos, normal);
            break;
        }
        case MATERIAL_ASPHALT:
        {
            col = texture_map_triplanar(asphalt_texture, ray_hit_pos, normal);
            break;
        }
        default:
        break;
    }
    return col;
}

void main()
{
    vec3 ray_view_dir = ray_view_dir(reso_time.xy, gl_FragCoord.xy);
    mat4 view_to_world = view_matrix(cam_pos, cam_dir);
    vec3 ray_world_dir = (view_to_world * vec4(ray_view_dir, 0.0)).xyz;

    Distance d = ray_march(cam_pos, ray_world_dir);

    vec3 ray_hit_pos = cam_pos + d.distance * ray_world_dir;
    vec3 normal = get_normal(ray_hit_pos);

    vec3 color = vec3(0.);
    if (d.distance > MAX_DISTANCE - EPS) {
        vec3 sky_color;
        switch (queuecount_raymarchmaxstep_aostep_background_type.w) {
            case SCENE_FOREST:
            {
                float h_f = SKY_HEIGHT / ray_world_dir.y;
                vec3 ray_hit_sky_pos = cam_pos + h_f * ray_world_dir;
                sky_color = clouds(ray_hit_sky_pos);
                break;
            }
            case SCENE_CITY:
            {
                sky_color = get_night_sky(ray_world_dir);
                break;
            }
            default:
            break;
        }
        color = mix(FOG_COLOR, sky_color, smoothstep(0., 1., ray_world_dir.y + .5));
        outColor = vec4(color, 1.);
    } else {
        color = color_mapping(ray_hit_pos, normal, d);
        const vec3 ambient_light = SKYCOLOR * color * 0.3;
        vec3 post_color = ambient_light;
        post_color += blinn_phong(
            color,
            vec3(1.0),
            10.0,
            ray_hit_pos,
            cam_pos,
            queuecount_raymarchmaxstep_aostep_background_type.w == SCENE_CITY ? vec3(0.0, 80.0, 0.0) : vec3(0.0, 20.0, 0.0),
            vec3(0.4),
            normal);
        #if DEBUG_POSITION == 1
        post_color += ray_hit_pos * 0.3;
        #endif
        post_color *= ambient_ocl(ray_hit_pos, normal);
        post_color = apply_fog(post_color, d.distance);
        outColor = vec4(post_color, 1.0);
    }

}
