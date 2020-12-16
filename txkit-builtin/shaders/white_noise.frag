#version 460 core

/**
 * @file white_noise.frag
 * @brief 3D uniform white noise
 * @author Vincent Tavernier <vince.tavernier@gmail.com>
 *
 * 3D uniform white noise
 */

layout(location = 0) in vec3 uv;
layout(location = 0) out vec4 o_FragColor;

#include "noise.glsl"
#include "shared.glsl"

void main() {
    ivec3 px = ivec3(uv * vec3(iResolution));

    uvec2 idx = shl64(
        mul64(add64(uvec2(0, px.x), uvec2(0, globalSeed)),
              uvec2(0, iResolution.x), uvec2(0, px.y + px.z * iResolution.y))
            .zw,
        2);

    o_FragColor = vec4(tofloat(hash64(idx | uvec2(0, 0)).x),
                       tofloat(hash64(idx | uvec2(0, 1)).x),
                       tofloat(hash64(idx | uvec2(0, 2)).x),
                       tofloat(hash64(idx | uvec2(0, 3)).x));
}

// vim: ft=glsl.doxygen
