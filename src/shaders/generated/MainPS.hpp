#ifndef SHADERS_MAINPS
#define SHADERS_MAINPS
const char* MAINPS = "#version 450\n"\
"\n"\
"#define DEV\n"\
"#define EPS 0.001\n"\
"\n"\
"uniform vec2 uResolution;\n"\
"uniform float uTime;\n"\
"uniform vec3 uCameraPosition;\n"\
"#ifdef DEV\n"\
"vec3 uCameraDirection = vec3(0.0, 0.0, -1.0);\n"\
"#else\n"\
"uniform vec3 uCameraDirection;\n"\
"#endif\n"\
"\n"\
"out vec4 outColor;\n"\
"\n"\
"struct RayHit\n"\
"{\n"\
"	vec3 pos;\n"\
"	float dist;\n"\
"};\n"\
"\n"\
"float sd_plane(vec3 pos, vec3 n, float h)\n"\
"{\n"\
"	return dot(pos, n) + h;\n"\
"}\n"\
"\n"\
"float sd_sphere(vec3 pos, vec3 center, float rad)\n"\
"{\n"\
"	return length(pos - center) - rad;\n"\
"}\n"\
"\n"\
"float sd_round_box(vec3 p, vec3 b)\n"\
"{\n"\
"  vec3 q = abs(p) - b;\n"\
"  return length(max(q, 0.0)) + min(max(q.x ,max(q.y,q.z)), 0.0);\n"\
"}\n"\
"\n"\
"float sd_box(vec3 pos, vec3 size)\n"\
"{\n"\
"	vec3 d = abs(pos) - size;\n"\
"	return max(d.x, max(d.y, d.z));\n"\
"}\n"\
"\n"\
"mat3 rot_z(float angle)\n"\
"{\n"\
"	float s = sin(angle);\n"\
"	float c = cos(angle);\n"\
"	return mat3(c, -s, 0, s, c, 0, 0, 0, 1);\n"\
"}\n"\
"\n"\
"float smin( float a, float b, float k )\n"\
"{\n"\
"    float h = clamp( 0.5+0.5*(b-a)/k, 0.0, 1.0 );\n"\
"    return mix( b, a, h ) - k*h*(1.0-h);\n"\
"}\n"\
"\n"\
"float sd_silver_horn(vec3 pos, vec3 c)\n"\
"{\n"\
"	pos -= c;\n"\
"	vec3 body_p = pos;\n"\
"\n"\
"	float body = sd_round_box(\n"\
"		body_p, vec3(1.60, 0.30, 0.10));\n"\
"\n"\
"	float in_pl = dot(pos - vec3(1, 1.25, 0), normalize(vec3(-0.20, 1, 0))) + 1.5;\n"\
"\n"\
"	body = max(body, -(dot(pos, normalize(vec3(1, 0.08, 0))) + 1.55));\n"\
"	body = max(body, -(dot(pos, normalize(vec3(-1, -0.45, 0))) + 1.40));\n"\
"	body = max(body, -in_pl);\n"\
"	body = max(body, -(dot(vec3(pos.x, abs(pos.yz)) - vec3(0, 0.30, 0.10), normalize(vec3(0,-1,-1))) - 0.05));\n"\
"\n"\
"	vec3 hp = pos - vec3(1.30, -.80, .0);\n"\
"	hp = rot_z(radians(20)) * hp;\n"\
"\n"\
"	float handle =  sd_round_box(\n"\
"		hp, vec3(0.30, 0.75, 0.10));\n"\
"		\n"\
"	handle = max(handle, -(dot(vec3(abs(hp.x), hp.y, abs(hp.z)) - vec3(0.30, 0.0, 0.10), normalize(vec3(-1, 0, -1))) - 0.04));\n"\
"	\n"\
"	float main_part = min(handle, body);\n"\
"	//main_part = max(main_part, -(dot(vec3(pos.xy, abs(pos.z)), normalize(vec3(0, 0, -1))) + 0.10));\n"\
"\n"\
"	vec3 tp = pos - vec3(0.60, -0.45, 0);\n"\
"	float trigger_frame = sd_box(tp, vec3(0.50, 0.20, 0.05));\n"\
"	\n"\
"	trigger_frame = max(trigger_frame, -(dot(tp - vec3(-0.50, 0.20, 0.05), normalize(vec3(1, 0.2, 0)))));\n"\
"	\n"\
"	vec3 hole_p = pos - vec3(0.50, -0.46, 0);\n"\
"	hole_p.xy = abs(hole_p.xy);\n"\
"	\n"\
"	float trigger_hole = length(hole_p.xy - vec2(clamp(hole_p.x, 0.0, 0.14), 0)) - 0.16;\n"\
"\n"\
"	//pos.x -= pos.y * pos.y;\n"\
"	//pos.z *= 1.5;\n"\
"	\n"\
"	return min(main_part, max(trigger_frame, -trigger_hole)); //length(pos.xz) - 0.2;\n"\
"    //return length(pos.xz) - 0.2;\n"\
"}\n"\
"\n"\
"vec3 repeat(vec3 pos, vec3 c)\n"\
"{\n"\
"	return mod(pos+0.5*c,c)-0.5*c;\n"\
"}\n"\
"\n"\
"float scene_dist(vec3 pos)\n"\
"{\n"\
"	float sph = sd_sphere(pos, vec3(0, 0.5, 0.), 0.5);\n"\
"	float m = min(sd_silver_horn(pos, vec3(0, 3, 0)), sph);\n"\
"	return min(m, sd_plane(pos, vec3(0, 1, 0),0));\n"\
"	//return sd_plane(pos, vec3(1, 0, 0), 0.5);\n"\
"}\n"\
"\n"\
"vec3 get_normal(vec3 pos)\n"\
"{\n"\
"	float center_dist = scene_dist(pos);\n"\
"	float dfx = scene_dist(vec3(pos.x - EPS, pos.y, pos.z));\n"\
"	float dfy = scene_dist(vec3(pos.x, pos.y - EPS, pos.z));\n"\
"	float dfz = scene_dist(vec3(pos.x, pos.y, pos.z - EPS));\n"\
"	vec3 normal = (center_dist - vec3(dfx, dfy, dfz)) / EPS;\n"\
"	\n"\
"	return normal;\n"\
"}\n"\
"\n"\
"float ray_march(vec3 ray_origin, vec3 ray_dir)\n"\
"{\n"\
"	float dist_traveled = 0.0;\n"\
"	vec3 current_pos = vec3(0);\n"\
"	\n"\
"	for (uint i = 0u; i < 100u; i++) {\n"\
"		current_pos = ray_origin + dist_traveled * ray_dir;\n"\
"		float closest_distance = scene_dist(current_pos);\n"\
"		\n"\
"		dist_traveled += closest_distance;\n"\
"\n"\
"		if (abs(closest_distance) < 0.00001 || dist_traveled > 1000.0f) {\n"\
"			break;\n"\
"		}\n"\
"	}\n"\
"\n"\
"	return dist_traveled;\n"\
"}\n"\
"\n"\
"float soft_shadow( in vec3 ro, in vec3 rd, float mint, float maxt, float k )\n"\
"{\n"\
"    float res = 1.0;\n"\
"    float ph = 1e10;\n"\
"    for( float t=mint; t<maxt; )\n"\
"    {\n"\
"        float h = scene_dist(ro + rd*t);\n"\
"        if( h<0.001 )\n"\
"            return 0.0;\n"\
"        float y = h*h/(2.0*ph);\n"\
"        float d = sqrt(h*h-y*y);\n"\
"        res = min( res, k*d/max(0.0,t-y) );\n"\
"        ph = h;\n"\
"        t += h;\n"\
"    }\n"\
"    return res;\n"\
"}\n"\
"\n"\
"float ambient_ocl( in vec3 pos, in vec3 nor )\n"\
"{\n"\
"	float occ = 0.0;\n"\
"    float sca = 1.0;\n"\
"    for( int i=0; i<5; i++ )\n"\
"    {\n"\
"        float h = 0.001 + 0.15*float(i)/4.0;\n"\
"        float d = scene_dist( pos + h*nor );\n"\
"        occ += (h-d)*sca;\n"\
"        sca *= 0.95;\n"\
"    }\n"\
"    return clamp( 1.0 - 1.5*occ, 0.0, 1.0 );\n"\
"}\n"\
"\n"\
"float light(vec3 n, vec3 lp, vec3 l)\n"\
"{\n"\
"	return clamp(dot(n, normalize(l - lp)), 0, 1);\n"\
"}\n"\
"\n"\
"vec3 lookat(vec2 uv, vec3 pos, vec3 dir, vec3 up)\n"\
"{\n"\
"	vec3 cam_dir = normalize(dir - pos);\n"\
"	vec3 right = normalize(cross(up, cam_dir));\n"\
"	vec3 cam_up = normalize(cross(cam_dir, right));\n"\
"	return normalize(uv.x * right + uv.y * cam_up + cam_dir * 2.0);\n"\
"}\n"\
"\n"\
"void main()\n"\
"{\n"\
"    vec2 uv = gl_FragCoord.xy/uResolution * 2.0 - 1.0;\n"\
"    uv.x *= uResolution.x/uResolution.y;\n"\
"    \n"\
"    //uv *= 0.5;\n"\
"    \n"\
"    vec3 light_pos = vec3(sin(uTime) * 8, 8.0, -cos(uTime) * 8);\n"\
"    vec3 cam_pos = uCameraPosition; // vec3(sin(uTime) * 8, 3.0, -cos(uTime) * 8);\n"\
"    //vec3 cam_pos = vec3(0, 10, 0.1);\n"\
"    vec3 cam_dir = uCameraDirection;\n"\
"    vec3 cam_up = vec3(0.0, 1.0, 0.0);\n"\
"    vec3 dir = lookat(uv, cam_pos, cam_pos + cam_dir, cam_up);\n"\
"    //vec3 cam_dir = normalize(vec3(sin(uTime), 2.0, cos(uTime)) - cam_pos);\n"\
"    vec3 ray_dir = dir;\n"\
"    \n"\
"    float d = ray_march(cam_pos, ray_dir);\n"\
"    vec3 lp = cam_pos + d * ray_dir;\n"\
"    vec3 normal = get_normal(lp);\n"\
"    float lv = 0.;\n"\
"    \n"\
"    lv += light(normal, lp, light_pos) * 0.7;\n"\
"    \n"\
"    lv *= ambient_ocl(lp, normal);\n"\
"    \n"\
"    lv += 0.05;\n"\
"    \n"\
"    outColor = vec4(clamp(pow(vec3(0.8, 0.9, 1.0) * lv, 1./vec3(2.2)), 0, 1), 1.0);\n"\
"    //outColor = vec4(ray_dir, 1.0);\n"\
"}"; 
#endif