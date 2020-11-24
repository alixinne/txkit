#define M_PI 3.14159265358979323846

layout(location = 0) uniform uvec3 iResolution;

// Simple coordinate hash
uint hash(uint x) {
    x = ((x >> 16) ^ x) * 0x45d9f3bu;
    x = ((x >> 16) ^ x) * 0x45d9f3bu;
    x = (x >> 16) ^ x;
    return x;
}

uint morton(uint x, uint y) {
    uint z = 0;
    for (int i = 0; i < 32 * 4; i++) {
        z |= ((x & (1 << i)) << i) | ((y & (1 << i)) << (i + 1));
    }
    return z;
}

// vec2 version
uint hash(uvec2 x, uint seed) { return hash(seed + morton(x.x, x.y)); }

// vec2 -> vec2 version
uvec2 hash2(uvec2 x, uint seed) {
    return uvec2(hash(seed + 2 * x.x), hash(seed + 2 * x.y + 1));
}

// Converts a vector of unsigned ints to floats in [0,1]
float tofloat(uint u) {
    // Slower, but generates all dyadic rationals of the form k / 2^-24 equally
    // return vec4(u >> 8) * (1. / float(1u << 24));

    // Faster, but only generates all dyadic rationals of the form k / 2^-23
    // equally
    return uintBitsToFloat(0x7Fu << 23 | u >> 9) - 1.;
}

// See tofloat
vec2 tofloat(uvec2 u) { return vec2(tofloat(u.x), tofloat(u.y)); }

float tofloat11(uint u) { return 2. * tofloat(u) - 1.; }
vec2 tofloat11(uvec2 u) { return 2. * tofloat(u) - 1.; }

float to01(float x) { return .5 * x + .5; }
