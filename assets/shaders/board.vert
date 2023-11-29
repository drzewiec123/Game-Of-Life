#version 430 core

layout (location = 0) in vec2 inPosition;

uniform mat2 uTransform;

out vec2 boardPos;

void main()
{
    boardPos = (inPosition + vec2(1.0, 1.0)) / 2;
    gl_Position = vec4(uTransform * inPosition, 0.0, 1.0);
}