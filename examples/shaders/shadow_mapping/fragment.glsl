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
    
    // Get texture size for the shadow map
    vec2 texelSize = 1.0 / textureSize(shadowMap, 0);
    
    // Calculate the exact texel location
    vec2 shadowTexCoord = projCoords.xy;
    
    // PCF with bilinear filtering
    float shadow = 0.0;
    float weightSum = 0.0;
    
    for(int x = -1; x <= 1; ++x) {
        for(int y = -1; y <= 1; ++y) {
            // Calculate sample position
            vec2 samplePos = shadowTexCoord + vec2(x, y) * texelSize;
            vec2 sampleMapPos = samplePos / texelSize;
            
            // Get fractional part for bilinear weights
            vec2 fractPart = fract(sampleMapPos);
            vec2 floorPart = floor(sampleMapPos);
            
            // Sample four texels for bilinear interpolation
            float tl = texture(shadowMap, (floorPart + vec2(0.0, 1.0)) * texelSize).r;
            float tr = texture(shadowMap, (floorPart + vec2(1.0, 1.0)) * texelSize).r;
            float bl = texture(shadowMap, (floorPart + vec2(0.0, 0.0)) * texelSize).r;
            float br = texture(shadowMap, (floorPart + vec2(1.0, 0.0)) * texelSize).r;
            
            // Bilinear interpolation
            float top = mix(tl, tr, fractPart.x);
            float bottom = mix(bl, br, fractPart.x);
            float sampledDepth = mix(bottom, top, fractPart.y);
            
            // Gaussian weighting for PCF
            float weight = exp(-(x*x + y*y) / 2.0);
            shadow += ((currentDepth - bias > sampledDepth) ? 1.0 : 0.0) * weight;
            weightSum += weight;
        }
    }
    
    return shadow / weightSum;
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