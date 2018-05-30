#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec3 inColor;
layout(location = 2) in vec2 inTexCoord;

layout(location = 1) out vec3 fragColor;
layout(location = 0) out vec2 fragTexCoord;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    //const mat4 proj = mat4(vec4(1.0, 0.0, 0.0, 0.7), vec4(0.0, 1.0, 0.0, 0.7), vec4(0.0, 0.0, 1.0, 0.7), vec4(0.0, 0.0, 0.0, 1.0));
    const float Pi = 3.1415926535897932384626433832795;
    const float a = 0.3 * Pi/2;
    const float b = 0.3 * Pi/2;
    const float c = 0.3 * Pi/2;
    const mat4 ident = mat4(vec4(1.0, 0.0, 0.0, 0.0),
                           vec4(0.0, 1.0, 0.0, 0.0),
                           vec4(0.0, 0.0, 1.0, 0.0),
                           vec4(0.0, 0.0, 0.0, 1.0));
    const mat4 a4 = mat4(vec4(1.0, 0.0, 0.0, 0.0),
                         vec4(0.0, cos(a), -sin(a), 0.0),
                         vec4(0.0, sin(a), cos(a), 0.0),
                         vec4(0.0, 0.0, 0.0, 1.0));
    const mat4 b4 = mat4(vec4(cos(b), 0.0, sin(b), 0.0),
                        vec4(0.0, 1.0, 0.0, 0.0),
                        vec4(-sin(b), 0.0, cos(b), 0.0),
                        vec4(0.0, 0.0, 0.0, 1.0));
    const mat4 c4 = mat4(vec4(cos(c), -sin(c), 0.0, 0.0),
                        vec4(sin(c), cos(c), 0.0, 0.0),
                        vec4(0.0, 0.0, 1.0, 0.0),
                        vec4(0.0, 0.0, 0.0, 1.0));

    gl_Position = /*ubo.proj * ubo.view * ubo.model **/ ident * a4 * b4 * c4 * vec4(inPosition, 1.0);
    fragColor = inColor;
    fragTexCoord = inTexCoord;
}
