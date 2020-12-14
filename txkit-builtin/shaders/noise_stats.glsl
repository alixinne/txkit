#ifndef _NOISE_STATS_GLSL_
#define _NOISE_STATS_GLSL_

#include "shared.glsl"

#define STATS_MODE_NORMAL 0
#define STATS_MODE_PROCESS 1
#define STATS_MODE_LOOKAT 2

#define LATTICE_MODE_RECT_2D 0
#define LATTICE_MODE_SIMPLEX_2D 1

#ifndef PARAM_SCALE
#define PARAM_SCALE scale
layout(location = 52) uniform float scale;
#endif /* PARAM_SCALE */

layout(location = 50) uniform int statsMode;
layout(location = 51) uniform vec2 statsLookAt;

struct LatticeNoiseSample {
    vec2 position;
    ivec2 cell;
    uint seed;
};

const float SIMPLEX_K1 = 0.366025404;  // (sqrt(3)-1)/2;
const float SIMPLEX_K2 = 0.211324865;  // (3-sqrt(3))/6;

LatticeNoiseSample latticeSample(vec2 position, int mode) {
    LatticeNoiseSample res;

    // Regular sampling mode inside the lattice
    res.seed = PARAM_GLOBAL_SEED;

    if (statsMode == STATS_MODE_NORMAL) {
        // Regular sampling mode is the default
        position *= PARAM_SCALE;
    } else {
        // For process and look-at mode: each pixel being evaluated is seeded
        // differently
        uvec2 px = uvec2(position * iResolution.xy);

        // Generate one seed per pixel, mix it with the base seed, then hash it
        res.seed = hash(morton(morton(px.x, px.y), res.seed));

        if (statsMode == STATS_MODE_PROCESS) {
            // In process mode, evaluate at random positions for each pixel
            position = tofloat(hash2(px, res.seed * 32165431u)) * PARAM_SCALE;
        } else if (statsMode == STATS_MODE_LOOKAT) {
            // Look-at mode: all pixels describe the same position in a cell,
            // but are seeded differently
            position = statsLookAt;
        }
    }

    if (mode == LATTICE_MODE_RECT_2D) {
        res.position = fract(position);
        res.cell = ivec2(position);
    } else if (mode == LATTICE_MODE_SIMPLEX_2D) {
        res.cell = ivec2(position + (position.x + position.y) * SIMPLEX_K1);
        res.position = position - vec2(res.cell) + (res.cell.x + res.cell.y) * SIMPLEX_K2;
    }

    return res;
}

#endif /* _NOISE_STATS_GLSL_ */
