/**
 * @file lcg.frag
 * @brief Linear congruential pseudo-random number generator
 * @author Vincent Tavernier <vince.tavernier@gmail.com>
 */

#ifndef _LCG_GLSL_
#define _LCG_GLSL_

struct LCG {
    uint state;
};

LCG lcgSeed(uint seed) { return LCG(seed); }

uint lcgNext(inout LCG lcg) {
    uint x = (1103515245 * lcg.state + 12345) % (1 << 31);
    return x >> 16;
}

float lcgNext01(inout LCG lcg) { return lcgNext(lcg) / 32767.; }

float lcgNext11(inout LCG lcg) { return 2. * lcgNext01(lcg) - 1.; }

int lcgPoisson(inout LCG lcg, float mean) {
    int em = 0;

    if (em < 50.) {
        // Knuth for small means
        float g = exp(-mean);
        float t = lcgNext01(lcg);
        while (t > g) {
            ++em;
            t *= lcgNext01(lcg);
        }
    } else {
        // Gaussian approximation
        float u =
            sqrt(-2. * log(lcgNext01(lcg))) * cos(2. * M_PI * lcgNext01(lcg));

        // Scale
        em = int((u * sqrt(mean)) + mean + .5);
    }

    return em;
}

#endif /* _LCG_GLSL_ */

// vim: ft=glsl.doxygen
