#version 330

layout(location = 0) out vec4 outColor;

void main()
{
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = gl_FragCoord.xy / vec2(300.0);

    // Time varying pixel color
    vec3 col = 0.5 + 0.5*cos(uv.xyx+vec3(0,2,4));

    outColor = vec4(col,1.0);
}
