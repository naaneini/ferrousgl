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
    // Multiple waves for more complex water surface
    float waveHeight1 = 0.15;
    float waveLength1 = 1.5;
    float speed1 = 1.2;
    
    float waveHeight2 = 0.1;
    float waveLength2 = 0.8;
    float speed2 = 0.8;
    
    float waveHeight3 = 0.05;
    float waveLength3 = 0.3;
    float speed3 = 0.5;

    vec3 displacedPos = aPos;
    displacedPos.y += waveHeight1 * sin(aPos.x * waveLength1 + time * speed1) *
                     cos(aPos.z * waveLength1 * 0.7 + time * speed1);
    displacedPos.y -= waveHeight2 * sin(aPos.x * waveLength2 * 1.3 + time * speed2 + 1.0) *
                     cos(aPos.z * waveLength2 * 1.7 + time * speed2 + 2.0);
    displacedPos.y += waveHeight3 * sin(aPos.x * waveLength3 * 2.1 + time * speed3 + 3.0) *
                     cos(aPos.z * waveLength3 * 2.5 + time * speed3 + 4.0);

    // Calculate normal from all waves
    float dx1 = waveHeight1 * waveLength1 * cos(aPos.x * waveLength1 + time * speed1) *
               cos(aPos.z * waveLength1 * 0.7 + time * speed1);
    float dz1 = -waveHeight1 * waveLength1 * 0.7 * sin(aPos.x * waveLength1 + time * speed1) *
               sin(aPos.z * waveLength1 * 0.7 + time * speed1);
    
    float dx2 = waveHeight2 * waveLength2 * 1.3 * cos(aPos.x * waveLength2 * 1.3 + time * speed2 + 1.0) *
               cos(aPos.z * waveLength2 * 1.7 + time * speed2 + 2.0);
    float dz2 = -waveHeight2 * waveLength2 * 1.7 * sin(aPos.x * waveLength2 * 1.3 + time * speed2 + 1.0) *
               sin(aPos.z * waveLength2 * 1.7 + time * speed2 + 2.0);
    
    float dx3 = waveHeight3 * waveLength3 * 2.1 * cos(aPos.x * waveLength3 * 2.1 + time * speed3 + 3.0) *
               cos(aPos.z * waveLength3 * 2.5 + time * speed3 + 4.0);
    float dz3 = -waveHeight3 * waveLength3 * 2.5 * sin(aPos.x * waveLength3 * 2.1 + time * speed3 + 3.0) *
               sin(aPos.z * waveLength3 * 2.5 + time * speed3 + 4.0);

    vec3 normal = normalize(vec3(-(dx1 + dx2 + dx3), 1.0, -(dz1 + dz2 + dz3)));

    FragPos = vec3(model * vec4(displacedPos, 1.0));
    Normal = mat3(transpose(inverse(model))) * normal;

    gl_Position = projection * view * model * vec4(displacedPos, 1.0);
}