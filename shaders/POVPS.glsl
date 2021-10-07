#version 450

#define DEV
#define EPS 0.001

uniform vec2 uResolution;
uniform float uTime;
uniform vec3 uCameraPosition;
#ifdef DEV
vec3 uCameraDirection = vec3(0.0, 0.0, -1.0);
#else
uniform vec3 uCameraDirection;
#endif

out vec4 outColor;

struct RayHit
{
	vec3 pos;
	float dist;
};

float sd_plane(vec3 pos, vec3 n, float h)
{
	return dot(pos, n) + h;
}

float sd_sphere(vec3 pos, vec3 center, float rad)
{
	return length(pos - center) - rad;
}

float sd_round_box(vec3 p, vec3 b)
{
  vec3 q = abs(p) - b;
  return length(max(q, 0.0)) + min(max(q.x ,max(q.y,q.z)), 0.0);
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

float smin( float a, float b, float k )
{
    float h = clamp( 0.5+0.5*(b-a)/k, 0.0, 1.0 );
    return mix( b, a, h ) - k*h*(1.0-h);
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
	body = max(body, -(dot(vec3(pos.x, abs(pos.yz)) - vec3(0, 0.30, 0.10), normalize(vec3(0,-1,-1))) - 0.05));

	vec3 hp = pos - vec3(1.30, -.80, .0);
	hp = rot_z(radians(20)) * hp;

	float handle =  sd_round_box(
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
    //return length(pos.xz) - 0.2;
}

vec3 repeat(vec3 pos, vec3 c)
{
	return mod(pos+0.5*c,c)-0.5*c;
}

float scene_dist(vec3 pos)
{
	float sph = sd_sphere(pos, vec3(0, 0.5, 0.), 0.5);
	float m = min(sd_silver_horn(pos, vec3(0, 3, 0)), sph);
	return m;
	//return sd_plane(pos, vec3(1, 0, 0), 0.5);
}

vec3 get_normal(vec3 pos)
{
	float center_dist = scene_dist(pos);
	float dfx = scene_dist(vec3(pos.x - EPS, pos.y, pos.z));
	float dfy = scene_dist(vec3(pos.x, pos.y - EPS, pos.z));
	float dfz = scene_dist(vec3(pos.x, pos.y, pos.z - EPS));
	vec3 normal = (center_dist - vec3(dfx, dfy, dfz)) / EPS;
	
	return normal;
}

float ray_march(vec3 ray_origin, vec3 ray_dir)
{
	float dist_traveled = 0.0;
	vec3 current_pos = vec3(0);
	
	for (uint i = 0u; i < 100u; i++) {
		current_pos = ray_origin + dist_traveled * ray_dir;
		float closest_distance = scene_dist(current_pos);
		
		dist_traveled += closest_distance;

		if (abs(closest_distance) < 0.00001 || dist_traveled > 1000.0f) {
			break;
		}
	}

	return dist_traveled;
}

float soft_shadow( in vec3 ro, in vec3 rd, float mint, float maxt, float k )
{
    float res = 1.0;
    float ph = 1e10;
    for( float t=mint; t<maxt; )
    {
        float h = scene_dist(ro + rd*t);
        if( h<0.001 )
            return 0.0;
        float y = h*h/(2.0*ph);
        float d = sqrt(h*h-y*y);
        res = min( res, k*d/max(0.0,t-y) );
        ph = h;
        t += h;
    }
    return res;
}

float ambient_ocl( in vec3 pos, in vec3 nor )
{
	float occ = 0.0;
    float sca = 1.0;
    for( int i=0; i<5; i++ )
    {
        float h = 0.001 + 0.15*float(i)/4.0;
        float d = scene_dist( pos + h*nor );
        occ += (h-d)*sca;
        sca *= 0.95;
    }
    return clamp( 1.0 - 1.5*occ, 0.0, 1.0 );
}

float light(vec3 n, vec3 lp, vec3 l)
{
	return clamp(dot(n, normalize(l - lp)), 0, 1);
}

vec3 lookat(vec2 uv, vec3 pos, vec3 dir, vec3 up)
{
	vec3 cam_dir = normalize(dir - pos);
	vec3 right = normalize(cross(up, cam_dir));
	vec3 cam_up = normalize(cross(cam_dir, right));
	return normalize(uv.x * right + uv.y * cam_up + cam_dir * 2.0);
}

void main()
{
    vec2 uv = gl_FragCoord.xy/uResolution * 2.0 - 1.0;
    uv.x *= uResolution.x/uResolution.y;
    
    //uv *= 0.5;
    
    vec3 light_pos = vec3(sin(uTime) * 8, 8.0, -cos(uTime) * 8);
    vec3 cam_pos = uCameraPosition + vec3(1.); // vec3(sin(uTime) * 8, 3.0, -cos(uTime) * 8);
    //vec3 cam_pos = vec3(0, 10, 0.1);
    vec3 cam_dir = uCameraDirection;
    vec3 cam_up = vec3(0.0, 1.0, 0.0);
    vec3 dir = lookat(uv, cam_pos, cam_pos + cam_dir, cam_up);
    //vec3 cam_dir = normalize(vec3(sin(uTime), 2.0, cos(uTime)) - cam_pos);
    vec3 ray_dir = dir;
    
    float d = ray_march(cam_pos, ray_dir);
    
    if (d > 1000.0f) {
    	outColor = vec4(0.);
    	return;
    }
    
    vec3 lp = cam_pos + d * ray_dir;
    vec3 normal = get_normal(lp);
    float lv = 0.;
    
    lv += light(normal, lp, light_pos) * 0.7;
    
    lv *= ambient_ocl(lp, normal);
    
    lv += 0.05;
    
    outColor = vec4(clamp(pow(vec3(0.8, 0.9, 1.0) * lv, 1./vec3(2.2)), 0, 1), 1.0);
    //outColor = vec4(ray_dir, 1.0);
}