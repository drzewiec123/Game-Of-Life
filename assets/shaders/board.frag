#version 430 core

uniform layout(binding = 1) usampler2D uBoard;

in vec2 texPos;

out vec4 Color;

void main()
{
    float col = texture(uBoard, texPos).r;
    Color = vec4(col, col, col, 1.0);
}