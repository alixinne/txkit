#version 460 core

layout(location = 0) out vec3 uv;

void main() {
    uv = vec3((gl_VertexID << 1) & 2, gl_VertexID & 2, 0.);
    gl_Position = vec4(uv.xy * 2. - 1., 0., 1.);
}
