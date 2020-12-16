#version 460 core

/**
 * @file simplex_noise.frag
 * @brief 2D simplex noise fragment shader
 * @author inigo quilez
 * @author Vincent Tavernier <vince.tavernier@gmail.com>
 * @see https://www.shadertoy.com/view/Msf3WH
 * @todo Support cyclic coordinates
 *
 * 2D Simplex noise, adapted for use in txkit.
 */

layout(location = 0) in vec3 uv;
layout(location = 0) out vec4 o_FragColor;

// Required built-ins
#include "noise.glsl"
#include "shared.glsl"

// Noise statistics helpers
#include "noise_stats.glsl"

vec2 noisehash(uvec2 p, uint seed) { return tofloat11(hash2(p, seed)); }

float noise(LatticeNoiseSample s) {
    vec2 a = s.position;
    float m = step(a.y, a.x);
    vec2 o = vec2(m, 1.0 - m);
    vec2 b = a - o + SIMPLEX_K2;
    vec2 c = a - 1.0 + 2.0 * SIMPLEX_K2;
    vec3 h = max(0.5 - vec3(dot(a, a), dot(b, b), dot(c, c)), 0.0);
    vec3 n = h * h * h * h *
             vec3(dot(a, noisehash(s.cell + 0, s.seed)),
                  dot(b, noisehash(s.cell + uvec2(o), s.seed)),
                  dot(c, noisehash(s.cell + 1, s.seed)));
    return dot(n, vec3(71.2825901));
}

void main() {
    o_FragColor = vec4(
        vec3(to01(noise(latticeSample(uv.xy, LATTICE_MODE_SIMPLEX_2D)))), 1.0);
}

// vim: ft=glsl.doxygen
