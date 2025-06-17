#version 330 core

in vec2 TexCoord; // Texture coordinate from vertex shader
out vec4 FragColor; // Output color

uniform sampler2D ourTexture; // Texture sampler

void main() {
    FragColor = texture(ourTexture, TexCoord); // Sample texture color
}