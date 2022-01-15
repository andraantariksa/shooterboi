#version 450

#include "common.glsl"

float hash(float n)
{
    return fract(sin(n) * 43758.5453);
}

float hash1_repeat(vec2 p2, float scale)
{
	p2 = mod(p2, scale);
    p2 = fract(p2 * vec2(5.3983, 5.4427));
    p2 += dot(p2.yx, p2.xy + vec2(21.5351, 14.3137));
    return fract(p2.x * p2.y * 95.4337);
}

float noise(vec2 pos, float scale)
{
	pos *= scale;
	const vec2 perm = vec2(1.0, 0.0);
	vec2 tile_id = floor(pos);
    vec2 tile_pos = fract(pos);
    
    // get random values at four corners
    // c0 ------ c1
    //  |		  |
    //  |   	  |
    //  |		  |
    // c2 ------ c3
    float c0 = hash1_repeat(tile_id, scale);
    float c1 = hash1_repeat(tile_id + perm.xy, scale);
    float c2 = hash1_repeat(tile_id + perm.yx, scale);
    float c3 = hash1_repeat(tile_id + perm.xx, scale);
    
    // sample value between four corner random values
    // c0 ------ c1
    //  |		  |
    //  |  x	  | x = sample point
    //  |		  |
    // c2 ------ c3
    float m0_x = mix(c0, c1, tile_pos.x);
    float m1_x = mix(c2, c3, tile_pos.x);
    float m = mix(m0_x, m1_x, tile_pos.y);
    
    return smoothstep(0.3, 1.0, m);
    //return m;
}

float fbm(vec2 pos)
{
	float ns = 0.0;
	float oct = 6.0;
	float amp = 0.5;
	float scale = 3.0;
	vec2 p = mod(pos, scale);
	
	for (float f = 0.0; f < oct; f += 1.0) {
		ns += noise(p + vec2(hash(f * 1.0), hash(f * 4.0)), scale) * amp;
		amp *= 0.5;
		scale *= 2.0;
	}
	
	return ns;
}

void main()
{
    vec2 p = reso_time.xy / gl_FragCoord.xy;
    outColor = vec4(vec3(fbm(p)), 1.0);
}
