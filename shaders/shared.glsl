layout(location = 0) uniform uvec3 iResolution;

// Simple coordinate hash
uint hash(in uint x) {
    x = ((x >> 16) ^ x) * 0x45d9f3bu;
    x = ((x >> 16) ^ x) * 0x45d9f3bu;
    x = (x >> 16) ^ x;
    return x;
}

// Converts a vector of unsigned ints to floats in [0,1]
float tofloat(uint u) {
    // Slower, but generates all dyadic rationals of the form k / 2^-24 equally
    // return vec4(u >> 8) * (1. / float(1u << 24));

    // Faster, but only generates all dyadic rationals of the form k / 2^-23 equally
    return uintBitsToFloat(0x7Fu << 23 | u >> 9) - 1.;
}
