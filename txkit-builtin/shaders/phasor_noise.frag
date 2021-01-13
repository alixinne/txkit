#version 460 core

/**
 * @file phasor_noise.frag
 * @brief Phasor noise fragment shader
 * @author Vincent Tavernier <vince.tavernier@gmail.com>
 * @see https://hal.archives-ouvertes.fr/hal-02118508/
 * @see https://hal.inria.fr/hal-02524371/
 *
 * Phasor noise implementation without kernel optimization. Also
 * implements Gabor noise by using the relevant profile function.
 */

layout(location = 0) in vec3 uv;
layout(location = 0) out vec4 o_FragColor;

// Required built-ins
#include "noise.glsl"
#include "shared.glsl"

// Noise statistics helpers
#include "noise_stats.glsl"

// Fast rng
#include "lcg.glsl"

layout(location = 20) uniform int noise_lookahead;
layout(location = 21) uniform int kernel_count;
layout(location = 22) uniform int noise_profile;
layout(location = 23) uniform int noise_weights;
layout(location = 24) uniform int noise_point_distribution;

layout(location = 25) uniform float noise_frequency;
layout(location = 26) uniform float noise_angle;

layout(location = 27) uniform float jitter_amount;
layout(location = 28) uniform int jitter_max;

layout(location = 29, binding = 0, rgba32f) uniform image2D frequency_orientation_field;

#define PHASOR_PROFILE_IMPULSES 5

#define PHASOR_POINTS_RECT_JITTERED 2
#define PHASOR_POINTS_HEX_JITTERED 3

struct Kernel {
    vec2 pos;
    float frequency;
    float phase;
    float angle;
    float weight;
};

vec2 phasor(vec2 x, Kernel k) {
    if (noise_profile == PHASOR_PROFILE_IMPULSES) {
        return exp(-M_PI * (0.125 * iResolution.x / scale) * (0.125 * iResolution.x / scale) * dot(x, x)) * vec2(1., k.weight);
    }

    float gaus, osc;
    float b = (1. / scale) * (1. / scale) * M_PI;

    gaus = exp(-b * dot(x, x));
    osc = 2. * M_PI * dot(x, k.frequency * vec2(cos(k.angle), sin(k.angle))) +
          k.phase;

    return k.weight * gaus * vec2(cos(osc), sin(osc));
}

vec2 noiseCell(vec2 pos, ivec2 cell, uint seed) {
    vec2 res = vec2(0.);

    // Seed the random number generator
    LCG rng = lcgSeed(hash(cell, seed));

    // Compute impulse count
    int ic = kernel_count, tc;
    ivec2 bc;

    if (noise_point_distribution == 1 /* PHASOR_POINTS_POISSON */) {
        ic = lcgPoisson(rng, ic);
    } else if (noise_point_distribution == PHASOR_POINTS_RECT_JITTERED ||
            noise_point_distribution == PHASOR_POINTS_HEX_JITTERED) {
        bc.x = int(sqrt(ic));
        bc.x = jitter_max == 0 ? bc.x : min(jitter_max, bc.x);
        bc.y = jitter_max == 0 ? ic / bc.x : min(jitter_max, ic / bc.x);
        tc = bc.x * bc.y;
        ic = (ic / tc) * tc;
    }

    // Fixed number of impulses per cell
    for (int i = 0; i < ic; ++i) {
        // Generate a kernel
        Kernel k;
        k.pos = .5 + (vec2(lcgNext01(rng), lcgNext01(rng)) - .5) * jitter_amount;

        if (noise_point_distribution == PHASOR_POINTS_RECT_JITTERED ||
                noise_point_distribution == PHASOR_POINTS_HEX_JITTERED)
        {
            int col = (i % tc) % bc.x;
            int row = (i % tc) / bc.x;

            if (noise_point_distribution == PHASOR_POINTS_HEX_JITTERED)
            {
                // Triangle transform
                k.pos = vec2(.25 * (k.pos.x - k.pos.y), .5 * abs(k.pos.x + k.pos.y));

                // Offset one every two triangles
                if ((ic + row) % 2 == 0)
                    k.pos = vec2(k.pos.x + .5, 1. - k.pos.y);
            }

            k.pos = (k.pos + vec2(col, row)) / vec2(bc);
        }

        k.frequency = noise_frequency / scale;
        k.phase = 0.;
        k.angle = noise_angle;

        // Compute weighting: always step the generator so we can get the same
        // image with and without weights
        float v = lcgNext11(rng);
        k.weight = 1.;
        if (noise_weights == 1 /* PHASOR_WEIGHTS_BERNOULLI */) {
            k.weight = v < 0. ? -1. : 1.;
        } else if (noise_weights == 2 /* PHASOR_WEIGHTS_UNIFORM */) {
            k.weight = v;
        }

        // Compute contribution
        res += phasor(scale * (pos - k.pos), k);
    }

    return res;
}

vec3 noise(LatticeNoiseSample s) {
    // Compute complex phasor value
    vec2 res = vec2(0.);

    ivec2 cell = s.cell;
    for (cell.x = s.cell.x - noise_lookahead;
         cell.x <= s.cell.x + noise_lookahead; ++cell.x) {
        for (cell.y = s.cell.y - noise_lookahead;
             cell.y <= s.cell.y + noise_lookahead; ++cell.y) {
            // Make sure the noise tiles correctly
            ivec2 looped_cell = latticeLoop(cell);

            // Compute the position of the current point relative to the target
            // cell
            vec2 position =
                s.position - vec2(cell.x - s.cell.x, cell.y - s.cell.y);

            // Add contribution of target noise cell
            res += noiseCell(position, looped_cell, s.seed);
        }
    }

    // Apply profile
    if (noise_profile == 0 /* PHASOR_PROFILE_COMPLEX */) {
        return vec3(to01(res / kernel_count), 0.);
    } else if (noise_profile == 1 /* PHASOR_PROFILE_REAL */) {
        return vec3(to01(res.x / kernel_count));
    } else if (noise_profile == 2 /* PHASOR_PROFILE_IMAG */) {
        return vec3(to01(res.y / kernel_count));
    } else if (noise_profile == PHASOR_PROFILE_IMPULSES) {
        return vec3(res, min(s.position.x, s.position.y) < scale / iResolution.x ? 1. : 0.);
    } else {
        float ph = atan(res.x, res.y);

        if (noise_profile == 3 /* PHASOR_PROFILE_SIN */) {
            return vec3(to01(sin(ph)));
        } else if (noise_profile == 4 /* PHASOR_PROFILE_SAW */) {
            return vec3(mod(ph + M_PI, M_2PI) / M_2PI);
        }

        // Invalid enum value
        return vec3(1., 0., 1.);
    }
}

void main() {
    o_FragColor = vec4(noise(latticeSample(uv.xy, LATTICE_MODE_RECT_2D)), 1.);
}

// vim: ft=glsl.doxygen
