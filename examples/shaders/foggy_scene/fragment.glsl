#version 330 core
in vec2 TexCoord;
in float FragDistance;  // Received from vertex shader
out vec4 FragColor;

uniform sampler2D ourTexture;
uniform vec3 fogColor = vec3(0.52, 0.67, 1.0);  // Gray fog color
uniform float fogStart = 0.0;                 // Distance where fog starts
uniform float fogEnd = 15.0;                   // Distance where fog ends
uniform float fogDensity = 0.1;               // For exponential fog
uniform int fogMode = 2;                       // 0=linear, 1=exp, 2=exp2

void main() {
    vec4 texColor = texture(ourTexture, TexCoord);
    
    // Calculate fog factor
    float fogFactor = 1.0;
    
    if (fogMode == 0) {  // Linear fog
        fogFactor = (fogEnd - FragDistance) / (fogEnd - fogStart);
        fogFactor = clamp(fogFactor, 0.0, 1.0);
    }
    else if (fogMode == 1) {  // Exponential fog
        fogFactor = exp(-fogDensity * FragDistance);
        fogFactor = clamp(fogFactor, 0.0, 1.0);
    }
    else if (fogMode == 2) {  // Exponential squared fog
        fogFactor = exp(-pow(fogDensity * FragDistance, 2.0));
        fogFactor = clamp(fogFactor, 0.0, 1.0);
    }
    
    // Mix texture color with fog color
    FragColor = mix(vec4(fogColor, 1.0), texColor, fogFactor);
}