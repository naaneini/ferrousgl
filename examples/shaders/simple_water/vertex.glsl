#version 330 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 aNormal;

out vec3 FragPos;
out vec3 Normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform float time;

void main() {
    float waveHeight = 0.2;
    float waveLength = 2.0;
    float speed = 1.0;

    vec3 displacedPos = aPos;
    displacedPos.y += waveHeight * sin(aPos.x * waveLength + time * speed) *
        cos(aPos.z * waveLength + time * speed);

    float dx = waveHeight * waveLength * cos(aPos.x * waveLength + time * speed) *
        cos(aPos.z * waveLength + time * speed);
    float dz = -waveHeight * waveLength * sin(aPos.x * waveLength + time * speed) *
        sin(aPos.z * waveLength + time * speed);
    vec3 normal = normalize(vec3(-dx, 1.0, -dz));

    FragPos = vec3(model * vec4(displacedPos, 1.0));
    Normal = mat3(transpose(inverse(model))) * normal;

    gl_Position = projection * view * model * vec4(displacedPos, 1.0);
}