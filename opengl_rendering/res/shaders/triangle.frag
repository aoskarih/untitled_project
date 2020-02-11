#version 330 core

in VS_OUTPUT {
    vec3 Color;
} IN;
in vec4 gl_FragCoord;

out vec4 Color;

uniform vec2 iResolution;
uniform float iTime;

void main()
{
    //vec2 iResolution = vec2(1280, 720);
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = gl_FragCoord.xy/iResolution.xy;

    // Time varying pixel color
    vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));

    // Output to screen
    Color = vec4(col,1.0);
}