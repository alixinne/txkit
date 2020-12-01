#version 460 core

layout(location = 0) in vec3 uv;
layout(location = 0) out vec4 o_FragColor;

#include "noise.glsl"
#include "shared.glsl"

layout(location = 20) uniform float scale;

float noisehash(uvec2 p, uint seed) { return tofloat(hash(p, seed)); }

float noise(vec2 p, uint seed) {
    uvec2 i = uvec2(p);
    vec2 f = fract(p);

    vec2 u = f * f * (3. - 2. * f);

    return mix(mix(noisehash(i + uvec2(0, 0), seed),
                   noisehash(i + uvec2(1, 0), seed), u.x),
               mix(noisehash(i + uvec2(0, 1), seed),
                   noisehash(i + uvec2(1, 1), seed), u.x),
               u.y);
}

void main() { o_FragColor = vec4(vec3(noise(scale * uv.xy, globalSeed)), 1.0); }
