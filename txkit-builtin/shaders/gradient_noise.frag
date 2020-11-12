#version 460 core

layout(location = 0) in vec3 uv;
layout(location = 0) out vec4 o_FragColor;

#include "shared.glsl"

vec2 noisehash(vec2 x) {
    const vec2 k = vec2(0.3183099, 0.3678794);
    x = x * k + k.yx;
    return -1.0 + 2.0 * fract(16.0 * k * fract(x.x * x.y * (x.x + x.y)));
}

float noise(in vec2 p) {
    uvec2 i = uvec2(p);
    vec2 f = fract(p);

    vec2 u = f * f * (3. - 2. * f);

    return mix(mix(dot(noisehash(i + vec2(0, 0)), f - vec2(0., 0.)),
                   dot(noisehash(i + vec2(1, 0)), f - vec2(1., 0.)), u.x),
               mix(dot(noisehash(i + vec2(0, 1)), f - vec2(0., 1.)),
                   dot(noisehash(i + vec2(1, 1)), f - vec2(1., 1.)), u.x),
               u.y);
}

void main() { o_FragColor = vec4(vec3(to01(noise(32. * uv.xy))), 1.0); }
