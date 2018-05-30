#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 0) uniform texture2D u_Texture;
layout(set = 0, binding = 1) uniform sampler u_Sampler;

layout(location = 1) in vec3 fragColor;
layout(location = 0) in vec2 fragTexCoord;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(fragColor, 1.0);
    //outColor = texture(sampler2D(u_Texture, u_Sampler), fragTexCoord) * vec4(fragColor, 1.0);
}
