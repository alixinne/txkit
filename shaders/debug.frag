#version 460 core

layout(location = 0) in vec3 uv;
layout(location = 0) out vec4 o_FragColor;

#include "shared.glsl"

layout(location = 10) uniform float alpha_value;

void main() {
    ivec3 px = ivec3(uv * vec3(iResolution));

    o_FragColor = vec4(px, alpha_value);
}
