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

float sd_cone( in vec3 p, in vec2 c, float h)
{
    vec2 w = h*vec2(c.x/c.y,-1.0);
    vec2 q = vec2( length(p.xz), p.y );

    vec2 a = q - w*clamp( (q.x*w.x+q.y*w.y)/dot(w, w), 0.0, 1.0 );
    vec2 b = q - w*vec2( clamp( q.x/w.x, 0.0, 1.0 ), 1.0 );

    float s = -sign( w.y );
    vec2 d = min( vec2( dot( a, a ), s*(q.x*w.y-q.y*w.x) ),
    vec2( dot( b, b ), s*(q.y-w.y)  ));
    return -sqrt(d.x)*sign(d.y);
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