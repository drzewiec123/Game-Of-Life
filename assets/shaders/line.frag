#version 330 core

out vec4 Color;

uniform vec3 uColor;

void main()
{
    Color = vec4(uColor, 1.0);
}