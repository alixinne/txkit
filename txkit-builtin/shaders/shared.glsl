/**
 * @file shared.glsl
 * @brief Shared utilities for txkit shaders
 * @author Vincent Tavernier <vince.tavernier@gmail.com>
 */

#ifndef _SHARED_GLSL_
#define _SHARED_GLSL_

/// The pi mathematical constant
#define M_PI 3.14159265358979323846

/// Output image resolution
layout(location = 0) uniform uvec3 iResolution;

/**
 * @brief Low-bias 32 bit hash function
 * @param x Value to hash
 * @return Hashed value
 * @see https://github.com/skeeto/hash-prospector#three-round-functions
 */
uint hash(uint x) {
    x++;
    x = ((x >> 17) ^ x) * 0xed5ad4bbu;
    x = ((x >> 11) ^ x) * 0xac4c1b51u;
    x = ((x >> 15) ^ x) * 0x31848babu;
    x = ((x >> 14) ^ x);
    return x;
}

/**
 * @brief Insert a 0 bit after each of the 16 low bits of x
 * @param x Value to part
 * @see https://fgiesen.wordpress.com/2009/12/13/decoding-morton-codes/
 */
uint mortonPart1By1(uint x) {
    x &= 0x0000ffffu;
    x = (x ^ (x << 8)) & 0x00ff00ffu;
    x = (x ^ (x << 4)) & 0x0f0f0f0fu;
    x = (x ^ (x << 2)) & 0x33333333u;
    x = (x ^ (x << 1)) & 0x55555555u;
    return x;
}

/**
 * @brief Insert two 0 bits after each of the 16 low bits of x
 * @param x Value to part
 * @see https://fgiesen.wordpress.com/2009/12/13/decoding-morton-codes/
 */
uint mortonPart1By2(uint x) {
    x &= 0x000003ffu;
    x = (x ^ (x << 16)) & 0xff0000ffu;
    x = (x ^ (x << 8)) & 0x0300f00fu;
    x = (x ^ (x << 4)) & 0x030c30c3u;
    x = (x ^ (x << 2)) & 0x09249249u;
    return x;
}

/**
 * @brief Encode two coordinates in Morton order
 * @param x First parameter
 * @param y Second parameter
 * @return Encoded Morton value
 */
uint morton(uint x, uint y) {
    return (mortonPart1By1(y) << 1) | mortonPart1By1(x);
}

/**
 * @brief Encode three coordinates in Morton order
 * @param x First parameter
 * @param y Second parameter
 * @param z Third parameter
 * @return Encoded Morton value
 */
uint morton(uint x, uint y, uint z) {
    return (mortonPart1By2(z) << 2) | (mortonPart1By2(y) << 1) |
           mortonPart1By2(x);
}

/**
 * @brief Hash a (coordinates, seed) pair
 * @param x Coordinates
 * @param seed Random seed
 * @return Hashed value
 *
 * Note that the coordinates are enumerated in Morton order, then hashed along
 * with the seed. Thus, cell coordinates are limited to their low 8 bits. Same
 * thing for the seed which is limited to its low 16 bits.
 */
uint hash(uvec2 x, uint seed) {
    return hash((seed << 16) | (0x0000ffffu & morton(x.x, x.y)));
}

/**
 * @brief Hash a (coordinates, seed) pair, return two values
 * @param x Coordinates
 * @param seed Random seed
 * @return Hashed value
 *
 * See uint version for limitations.
 */
uvec2 hash2(uvec2 x, uint seed) {
    // Mix both coordinates into one seed value
    uint base = hash(seed + morton(x.x, x.y));
    // Hash both coordinates
    return uvec2(hash(2 * base), hash(2 * base + 1));
}

/**
 * @brief Convert an unsigned int to a float in [0, 1]
 * @param u Unsigned int to convert
 * @return float in [0, 1]
 */
float tofloat(uint u) {
    // Slower, but generates all dyadic rationals of the form k / 2^-24 equally
    // return vec4(u >> 8) * (1. / float(1u << 24));

    // Faster, but only generates all dyadic rationals of the form k / 2^-23
    // equally
    return uintBitsToFloat(0x7Fu << 23 | u >> 9) - 1.;
}

/**
 * @brief Convert a vector of unsigned ints to floats
 * @param u Vector to convert
 * @return vec in [0, 1]
 */
vec2 tofloat(uvec2 u) { return vec2(tofloat(u.x), tofloat(u.y)); }


/**
 * @brief Convert an unsigned int to a float in [-1, 1]
 * @param u Unsigned int to convert
 * @return float in [-1, 1]
 */
float tofloat11(uint u) { return 2. * tofloat(u) - 1.; }

/**
 * @brief Convert a vector of unsigned ints to floats
 * @param u Vector to convert
 * @return vec in [-1, 1]
 */
vec2 tofloat11(uvec2 u) { return 2. * tofloat(u) - 1.; }

/**
 * @brief Convert a float value from [-1, 1] to [0, 1]
 * @param x Value to convert
 * @return Value in [0, 1]
 */
float to01(float x) { return .5 * x + .5; }

#endif /* _SHARED_GLSL_ */

// vim: ft=glsl.doxygen
