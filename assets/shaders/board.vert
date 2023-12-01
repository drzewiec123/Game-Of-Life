#version 430 core

layout (location = 0) in vec2 inPosition;

uniform mat3 uTransform;
uniform mat3 uTexTransform;

out vec2 texPos;

void main()
{
    vec3 texture_pos = uTexTransform * vec3(inPosition, 1.0);
    texPos = (texture_pos.xy / texture_pos.z + vec2(1.0, 1.0)) / 2.0;
    vec3 pos = uTransform * vec3(inPosition, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}