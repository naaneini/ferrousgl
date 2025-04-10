#version 330 core
in vec2 TexCoords;
out vec4 FragColor;

uniform sampler2D textureAtlas;
uniform vec3 textColor;
uniform vec4 backgroundColor;
uniform int hasBackground;

void main() {
    float alpha = texture(textureAtlas, TexCoords).r;
    
    if (hasBackground == 1) {
        // Blend text with background
        FragColor = mix(
            vec4(backgroundColor.rgb, 0.0), // Transparent where text is
            vec4(textColor, backgroundColor.a * alpha),
            alpha
        );
    } else {
        // Just render text
        FragColor = vec4(textColor, alpha);
    }
}