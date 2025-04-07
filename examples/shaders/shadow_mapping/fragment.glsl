#version 330 core
out vec4 FragColor;

in vec2 TexCoords;
in vec3 FragPos;
in vec4 FragPosLightSpace;

uniform sampler2D diffuseTexture;
uniform sampler2D shadowMap;
uniform vec3 lightPos;
uniform vec3 viewPos;

uniform int shadowBlurKernelSize;
uniform vec3 lightColor;
uniform vec3 ambientColor;

float ShadowCalculation(vec4 fragPosLightSpace) {
    // Perform perspective divide
    vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;
    // Transform to [0,1] range
    projCoords = projCoords * 0.5 + 0.5;
    
    // Early exit if outside light frustum
    if(projCoords.z > 1.0)
        return 0.0;
    
    // Get depth of current fragment from light's perspective
    float currentDepth = projCoords.z;
    
    // Calculate bias (based on depth map resolution and slope)
    vec3 normal = normalize(cross(dFdx(FragPos), dFdy(FragPos)));
    float bias = max(0.05 * (1.0 - dot(normal, normalize(lightPos - FragPos))), 0.005);
    
    // PCF with dynamic kernel size
    float shadow = 0.0;
    vec2 texelSize = 1.0 / textureSize(shadowMap, 0);
    float weightSum = 0.0;
    
    for(int x = -shadowBlurKernelSize; x <= shadowBlurKernelSize; ++x)
    {
        for(int y = -shadowBlurKernelSize; y <= shadowBlurKernelSize; ++y)
        {
            // Gaussian-like weighting (optional)
            float weight = 1.0 / (1.0 + length(vec2(x,y)));
            
            float pcfDepth = texture(shadowMap, projCoords.xy + vec2(x, y) * texelSize).r; 
            shadow += (currentDepth - bias > pcfDepth ? 1.0 : 0.0) * weight;        
            weightSum += weight;
        }    
    }
    shadow /= weightSum;
    
    return shadow;
}

void main()
{           
    vec3 color = texture(diffuseTexture, TexCoords).rgb;
    vec3 normal = normalize(cross(dFdx(FragPos), dFdy(FragPos)));
    
    // Ambient
    vec3 ambient = 0.3 * ambientColor;
    
    // Diffuse
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(lightDir, normal), 0.0);
    vec3 diffuse = diff * lightColor;
    
    // Specular
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 halfwayDir = normalize(lightDir + viewDir);  
    float spec = pow(max(dot(normal, halfwayDir), 0.0), 64.0);
    vec3 specular = spec * lightColor;    
    
    // Calculate shadow
    float shadow = ShadowCalculation(FragPosLightSpace);                      
    vec3 lighting = (ambient + (1.0 - shadow) * (diffuse + specular)) * color;    
    
    FragColor = vec4(lighting, 1.0);
}