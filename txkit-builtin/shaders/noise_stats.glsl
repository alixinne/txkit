/**
 * @file noise_stats.glsl
 * @brief Noise statistics utilities
 * @author Alixinne <alixinne@pm.me>
 *
 * Lattice noise statistics utilities.
 */

#ifndef _NOISE_STATS_GLSL_
#define _NOISE_STATS_GLSL_

#include "shared.glsl"

/**
 * @enum SamplingMode
 * @brief Sampling mode for texture statistics
 */

/// Compute a regular procedural texture
#define STATS_MODE_NORMAL 0
/// Compute a process sample: each pixel is seeded differently, at random offset
#define STATS_MODE_PROCESS 1
/// Compute a local sample: each pixel is seeded differently, at a fixed offset
#define STATS_MODE_LOOKAT 2

/// Sample a rectangular lattice
#define LATTICE_MODE_RECT_2D 0
/// Sample a simplex lattice
#define LATTICE_MODE_SIMPLEX_2D 1

#ifndef PARAM_SCALE
/// Name of the scale uniform
#define PARAM_SCALE scale
/// Scale of the noise: how many lattice cells are visible in the current
/// viewport
layout(location = 52) uniform float scale;
#endif /* PARAM_SCALE */

/// Current statistics computation mode
layout(location = 50) uniform int statsMode;
/// Location for local sampling
layout(location = 51) uniform vec2 statsLookAt;

/// Sample in a 2D lattice
struct LatticeNoiseSample {
    /// Position in the current cell (in [0, 1])
    vec2 position;
    /// Cell number
    ivec2 cell;
    /// Seed for the current sample
    uint seed;
};

/// Simplex constant for (sqrt(3)-1)/2
const float SIMPLEX_K1 = 0.366025404;
/// Simplex constant for (3-sqrt(3))/6
const float SIMPLEX_K2 = 0.211324865;

/**
 * @brief Sample a 2D lattice
 * @param position Input position in the viewport
 * @param mode Sampling mode, see #SamplingMode
 * @return Sampled position structure
 */
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
        res.position =
            position - vec2(res.cell) + (res.cell.x + res.cell.y) * SIMPLEX_K2;
    }

    return res;
}

/**
 * @brief Ensure 2D lattice cells loop around the texture borders
 * @param p Cell coordinates to loop
 * @todo Support looping simplex grids
 */
ivec2 latticeLoop(ivec2 p) {
    const int S = int(PARAM_SCALE);

    if (statsMode == STATS_MODE_NORMAL) {
        return ivec2(p.x >= 0 ? p.x % S : S - (-p.x % S),
                     p.y >= 0 ? p.y % S : S - (-p.y % S));
    }

    return p;
}

/**
 * @brief Ensure 2D lattice cells loop around the texture borders
 * @param p Cell coordinates to loop
 * @todo Support looping simplex grids
 */
uvec2 latticeLoop(uvec2 p) {
    if (statsMode == STATS_MODE_NORMAL) {
        return p % int(PARAM_SCALE);
    }

    return p;
}

#endif /* _NOISE_STATS_GLSL_ */

// vim: ft=glsl.doxygen
