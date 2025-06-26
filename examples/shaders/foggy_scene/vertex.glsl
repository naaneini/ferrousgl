#version 330 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec2 aTexCoord;

out vec2 TexCoord;
out float FragDistance;  // Distance from camera to fragment

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 cameraPos;  // Camera position in world space

void main() {
    vec4 worldPos = model * vec4(aPos, 1.0);
    FragDistance = distance(worldPos.xyz, cameraPos);
    
    gl_Position = projection * view * worldPos;
    TexCoord = aTexCoord;
}