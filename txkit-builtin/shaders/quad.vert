#version 460 core

/**
 * @file quad.vert
 * @brief Buffer-less screen quad vertex shader
 * @author Morgan McGuire
 * @author Alixinne <alixinne@pm.me>
 * @see https://twitter.com/casualeffects/status/705750628849590273
 */

layout(location = 0) out vec3 uv;

layout(location = 0) uniform uvec3 iResolution;
layout(location = 1) uniform uint iLayer;

void main() {
    uv = vec3((gl_VertexID << 1) & 2, gl_VertexID & 2, (float(iLayer) + .5) / iResolution.z);
    gl_Position = vec4(uv.xy * 2. - 1., 0., 1.);
}

// vim: ft=glsl.doxygen
