float hash1(float p) {
    vec3 p3 = fract(p * vec3(5.3983, 5.4427, 6.9371));
    p3 += dot(p3, p3.yzx + 19.19);
    return fract((p3.x + p3.y) * p3.z);
}

float hash1(vec2 p2) {
    p2 = fract(p2 * vec2(5.3983, 5.4427));
    p2 += dot(p2.yx, p2.xy + vec2(21.5351, 14.3137));
    return fract(p2.x * p2.y * 95.4337);
}

float hash1(vec2 p2, float p) {
    vec3 p3 = fract(vec3(5.3983 * p2.x, 5.4427 * p2.y, 6.9371 * p));
    p3 += dot(p3, p3.yzx + 19.19);
    return fract((p3.x + p3.y) * p3.z);
}

vec2 hash2(vec2 p2, float p) {
    vec3 p3 = fract(vec3(5.3983 * p2.x, 5.4427 * p2.y, 6.9371 * p));
    p3 += dot(p3, p3.yzx + 19.19);
    return fract((p3.xx + p3.yz) * p3.zy);
}

vec3 hash3(vec2 p2) {
    vec3 p3 = fract(vec3(p2.xyx) * vec3(5.3983, 5.4427, 6.9371));
    p3 += dot(p3, p3.yxz + 19.19);
    return fract((p3.xxy + p3.yzz) * p3.zyx);
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
