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
 * @brief 64-bit addition
 * @param a First operand
 * @param b Second operand
 * @see https://www.bearssl.org/bigint.html#additions-and-subtractions-1
 */
uvec2 add64(uvec2 a, uvec2 b) {
    uint cc = 0, naw;

    // Low word
    naw = a.y + b.y + cc;

    cc = (cc & (naw == a.y ? 1 : 0)) | (naw < a.y ? 1 : 0);
    a.y = naw;

    // High word
    naw = a.x + b.x + cc;

    cc = (cc & (naw == a.x ? 1 : 0)) | (naw < a.x ? 1 : 0);
    a.x = naw;

    return a;
}

/**
 * @brief 64-bit left shift
 * @param a First operand
 * @param b Second operand
 * @see https://www.bearssl.org/bigint.html#additions-and-subtractions-1
 */
uvec2 shl64(uvec2 a, uint b) {
    a.x = (a.x << b) | (a.y >> (32 - b));
    a.y <<= b;
    return a;
}

/**
 * @brief 64-bit right shift
 * @param a First operand
 * @param b Second operand
 * @see https://www.bearssl.org/bigint.html#additions-and-subtractions-1
 */
uvec2 shr64(uvec2 a, uint b) {
    a.y = (a.y >> b) | (a.x << (32 - b));
    a.x >>= b;
    return a;
}

/**
 * @brief 64-bit multiplication and accumulation (d + a * b)
 * @param d First operand
 * @param a Second operand
 * @param b Third operand
 * @see https://www.bearssl.org/gitweb/?p=BearSSL;a=blob;f=src/int/i32_mulacc.c
 */
uvec4 mul64(uvec4 d, uvec2 a, uvec2 b) {
    uint f, v;
    uvec2 cc, z;

    {
        f = b.y;
        cc = uvec2(0);

        {
            z = add64(add64(uvec2(0, d.w), uvec2(0, f * a.y)), cc);
            cc = uvec2(0, z.x);  // z >> 32
            d.w = z.y;           // (uint32_t)z

            z = add64(add64(uvec2(0, d.z), uvec2(0, f * a.x)), cc);
            cc = uvec2(0, z.x);  // z >> 32
            d.z = z.y;           // (uint32_t)z
        }

        d.y = cc.y;  // (uint32_t)cc
    }

    {
        f = b.x;
        cc = uvec2(0);

        {
            z = add64(add64(uvec2(0, d.z), uvec2(0, f * a.y)), cc);
            cc = uvec2(0, z.x);  // z >> 32
            d.z = z.y;           // (uint32_t)z

            z = add64(add64(uvec2(0, d.y), uvec2(0, f * a.x)), cc);
            cc = uvec2(0, z.x);  // z >> 32
            d.y = z.y;           // (uint32_t)z
        }

        d.x = cc.y;  // (uint32_t)cc
    }

    return d;
}

/**
 * @brief 64-bit multiplication and accumulation (d + a * b)
 * @param d First operand
 * @param a Second operand
 * @param b Third operand
 * @see https://www.bearssl.org/gitweb/?p=BearSSL;a=blob;f=src/int/i32_mulacc.c
 */
uvec4 mul64(uvec2 d, uvec2 a, uvec2 b) {
    return mul64(uvec4(0, 0, d.x, d.y), a, b);
}

/**
 * @brief 64-bit hash function
 * @param x Low and high word of the 64-bit value
 * @return Hashed value
 * @see http://xoshiro.di.unimi.it/splitmix64.c
 */
uvec2 hash64(uvec2 x) {
    x = add64(x, uvec2(0x9e3779b9u, 0x7f4a7c15u));
    x = mul64(uvec4(0), x ^ shr64(x, 30), uvec2(0xbf58476du, 0x1ce4e5b9u)).zw;
    x = mul64(uvec4(0), x ^ shr64(x, 27), uvec2(0x94d049bbu, 0x133111ebu)).zw;
    return x ^ shr64(x, 31);
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
 * @brief Hash a (coordinates, seed) pair
 * @param x Coordinates
 * @param seed Random seed
 * @return Hashed value
 *
 * Note that the coordinates are enumerated in Morton order, then hashed along
 * with the seed. This version performs 64-bit arithmetic, thus, cell
 * coordinates are limited to their low 16 bits. The seed is utilized fully.
 */
uvec2 hash64(uvec2 x, uint seed) {
    return hash64(uvec2(seed, morton(x.x, x.y)));
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
