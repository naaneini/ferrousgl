#version 330 core
in vec3 FragPos;
in vec3 Normal;

out vec4 FragColor;

uniform vec3 lightPos;
uniform vec3 viewPos;
uniform vec3 lightColor;
uniform vec3 objectColor;

void main() {
    float ambientStrength = 0.5;
    vec3 ambient = ambientStrength * lightColor;

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    // Enhanced specular for water
    float specularStrength = 0.8;
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 128);
    vec3 specular = specularStrength * spec * lightColor;

    // Fresnel effect for more realistic water
    float fresnel = pow(1.0 - max(dot(norm, viewDir), 0.0), 2.0);
    vec3 waterColor = mix(objectColor, vec3(1.0), fresnel * 0.5);

    vec3 result = (ambient + diffuse + specular) * waterColor;
    FragColor = vec4(result, 0.7); // Semi-transparent
}