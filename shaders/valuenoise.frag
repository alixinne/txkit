#version 460 core

layout(location = 0) in vec3 uv;
layout(location = 0) out vec4 o_FragColor;

#include "shared.glsl"

float noise(in vec2 p) {
    uvec2 i = uvec2(p);
    vec2 f = fract(p);

    vec2 u = f * f * (3. - 2. * f);

    return mix(mix(tofloat(hash(i + uvec2(0, 0))),
                   tofloat(hash(i + uvec2(1, 0))), u.x),
               mix(tofloat(hash(i + uvec2(0, 1))),
                   tofloat(hash(i + uvec2(1, 1))), u.x),
               u.y);
}

void main() { o_FragColor = vec4(vec3(noise(32. * uv.xy)), 1.0); }
