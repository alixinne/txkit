#version 460 core

layout(location = 0) in vec3 uv;
layout(location = 0) out vec4 o_FragColor;

#include "noise.glsl"
#include "shared.glsl"

layout(location = 20) uniform float scale;

vec2 noisehash(uvec2 p, uint seed) {
    float x = 2. * M_PI * tofloat11(hash(p, seed));
    return vec2(cos(x), sin(x));
}

float noise(vec2 p, uint seed) {
    uvec2 i = uvec2(p);
    vec2 f = fract(p);

    vec2 u = f * f * (3. - 2. * f);

    return mix(
        mix(dot(noisehash(i + uvec2(0, 0), seed), f - uvec2(0., 0.)),
            dot(noisehash(i + uvec2(1, 0), seed), f - uvec2(1., 0.)), u.x),
        mix(dot(noisehash(i + uvec2(0, 1), seed), f - uvec2(0., 1.)),
            dot(noisehash(i + uvec2(1, 1), seed), f - uvec2(1., 1.)), u.x),
        u.y);
}

void main() {
    o_FragColor = vec4(vec3(to01(noise(scale * uv.xy, globalSeed))), 1.0);
}
