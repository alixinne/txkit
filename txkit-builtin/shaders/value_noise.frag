#version 460 core

layout(location = 0) in vec3 uv;
layout(location = 0) out vec4 o_FragColor;

// Required built-ins
#include "noise.glsl"
#include "shared.glsl"

// Noise statistics helpers
#include "noise_stats.glsl"

float noisehash(uvec2 p, uint seed) { return tofloat(hash(p, seed)); }

float noise(LatticeNoiseSample s) {
    uvec2 i = uvec2(s.cell);
    vec2 f = s.position;

    vec2 u = f * f * (3. - 2. * f);

    return mix(mix(noisehash(i + uvec2(0, 0), s.seed),
                   noisehash(i + uvec2(1, 0), s.seed), u.x),
               mix(noisehash(i + uvec2(0, 1), s.seed),
                   noisehash(i + uvec2(1, 1), s.seed), u.x),
               u.y) *
           0.73582062;
}

void main() {
    o_FragColor =
        vec4(vec3(noise(latticeSample(uv.xy, LATTICE_MODE_RECT_2D))), 1.0);
}
