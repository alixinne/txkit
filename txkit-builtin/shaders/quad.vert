#version 460 core

layout(location = 0) out vec3 uv;

layout(location = 0) uniform uvec3 iResolution;
layout(location = 1) uniform uint iLayer;

void main() {
    uv = vec3((gl_VertexID << 1) & 2, gl_VertexID & 2, (float(iLayer) + .5) / iResolution.z);
    gl_Position = vec4(uv.xy * 2. - 1., 0., 1.);
}
