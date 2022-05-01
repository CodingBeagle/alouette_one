// a "cbuffer" is a Constant Buffer.
// Constant buffers are blocks of memory which can store variables which can be 
// Accessed by a shader.
// Data is constant buffers doesn't vary per vertex, but stays the same.
cbuffer cbPerObject : register(b0)
{
    float4x4 worldViewProjection;
    float4x4 modelMatrix;
    float4 cameraPosition;
    float4 diffuseColor;
    float4 ambientColor;
    float4 specularColor;
};

struct VSIn
{
    float3 PosL : POSITION;
    float3 Normal : NORMAL;
};

struct VSOut
{
    float4 PosH : SV_POSITION;
    float4 Color : COLOR0;
    float3 Normal : NORMAL0;
};

VSOut VS(VSIn input)
{
    // Transform to homogenous clip space
    // Notice that the vertex shader, or any other shader, doesn't do the perspective divide.
    // The perspective divide is done by hardware at a later stage.
    // The vertex shader just does the projection matrix.
    // A vector on the LHS (PosL in this case) will be treated as a ROW VECTOR by HLSL.
    VSOut output;
    output.PosH = mul(float4(input.PosL, 1.0f), worldViewProjection);

    // ** Color is calculated using lightning equation **

    // Hardcoded sun light direction in world coordinates
    // Currently a noon-sun (directly overhead)
    // Notice that the light vector is opposite of where the light rays are actually coming from, which would be -1 on the Y axis.
    float3 sun_light_direction = { 0.0f, 1.0f, 0.0f };

    // ** Material parameters **

    // ** Light Parameters **

    // I want the diffuse light to be completely white
    float3 light_diffuse_color = { 1.0, 1.0, 1.0 };

    // I want the ambient light color to be completely white
    float3 light_ambient_color = { 1.0, 1.0, 1.0 };

    // Lambert's Cosine Law
    // We use this to calculate how intense the final color value should be, based on the surface's
    // angle to the incoming light direction
    // We use "max", an intrinsic HLSL function which selects whichever of x and y that are the largest
    float lamberts_multiplier = max(dot(sun_light_direction, input.Normal), 0.0f);
    float3 diffuse_calculation = light_diffuse_color * diffuseColor.xyz;
    float3 ambient_calculation = light_ambient_color * ambientColor.xyz;
    
    // ** Specular Light **
    float3 specular_light_color = float3(1.0, 1.0, 1.0);

    // A larger shiniess parameter will simulate more polished surfaces with smaller
    // cone of reflectance.
    // However, at a minimum, the parameter should always be 1 or greater.
    // Setting it to zero will simulate an object which receives no reflection / specular light
    float shininess_parameter = 5.0;

    // Right now I pass in a 4D vector for camera position in order to adhere to the 16 multiple requirement
    // of the vertex shader constants. I need to figure out a cleaner way of doing this, perhaps...
    float3 camera_position_truncated = float3(cameraPosition.x, cameraPosition.y, cameraPosition.z);
    float4 surface_point_in_world_coordinates = mul(float4(input.PosL, 1.0f), modelMatrix);

    float3 view_vector = normalize(camera_position_truncated - surface_point_in_world_coordinates.xyz);
    float3 reflection_vector = reflect(float3(0.0f, -1.0f, 0.0f), input.Normal);
    float3 specular_calculation = specular_light_color * specularColor.xyz;
    float specular_factor = pow(max(dot(reflection_vector, view_vector), 0.0f), shininess_parameter);

    if (lamberts_multiplier <= 0.0f)
    {
        specular_factor = 0.0f;
    }

    // It's important to note that the term of ambient color has NO physical simulations attached to it.
    // Meaning, the ambient color disregards any direction to the light source, because it's meant to simulate
    float3 lit_color = ambient_calculation + 
        float3(
            diffuse_calculation.x * lamberts_multiplier, 
            diffuse_calculation.y * lamberts_multiplier, 
            diffuse_calculation.z * lamberts_multiplier) +
        float3(
            specular_calculation.x * specular_factor,
            specular_calculation.y * specular_factor,
            specular_calculation.z * specular_factor
        );

    output.Color = float4(lit_color, 1.0);

    return output;
}