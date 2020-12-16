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
    uint idx = 4u * (px.x + px.y * iResolution.x +
                     px.z * iResolution.x * iResolution.y + globalSeed);

    o_FragColor = vec4(tofloat(hash(idx)), tofloat(hash(idx + 1)),
                       tofloat(hash(idx + 2)), tofloat(hash(idx + 3)));
}

// vim: ft=glsl.doxygen
