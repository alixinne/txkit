#version 460 core

/**
 * @file gradient_noise.frag
 * @brief 2D gradient noise fragment shader
 * @author inigo quilez
 * @author Alixinne <alixinne@pm.me>
 * @see https://www.shadertoy.com/view/XdXGW8
 *
 * 2D Gradient noise, adapted for use in txkit.
 */

layout(location = 0) in vec3 uv;
layout(location = 0) out vec4 o_FragColor;

// Required built-ins
#include "noise.glsl"
#include "shared.glsl"

// Noise statistics helpers
#include "noise_stats.glsl"

vec2 noisehash(uvec2 p, uint seed) {
    float x = 2. * M_PI * tofloat11(hash(p, seed));
    return vec2(cos(x), sin(x));
}

float noise(LatticeNoiseSample s) {
    uvec2 i = uvec2(s.cell);
    vec2 f = s.position;

    vec2 u = f * f * (3. - 2. * f);

    return mix(mix(dot(noisehash(latticeLoop(i + uvec2(0, 0)), s.seed),
                       f - uvec2(0, 0)),
                   dot(noisehash(latticeLoop(i + uvec2(1, 0)), s.seed),
                       f - uvec2(1, 0)),
                   u.x),
               mix(dot(noisehash(latticeLoop(i + uvec2(0, 1)), s.seed),
                       f - uvec2(0, 1)),
                   dot(noisehash(latticeLoop(i + uvec2(1, 1)), s.seed),
                       f - uvec2(1, 1)),
                   u.x),
               u.y) *
           1.49315244;
}

void main() {
    o_FragColor = vec4(
        vec3(to01(noise(latticeSample(uv.xy, LATTICE_MODE_RECT_2D)))), 1.0);
}

// vim: ft=glsl.doxygen
