// a "cbuffer" is a Constant Buffer.
// Constant buffers are blocks of memory which can store variables which can be 
// Accessed by a shader.
// Data is constant buffers doesn't vary per vertex, but stays the same.
cbuffer cbPerObject : register(b0)
{
    float4x4 worldViewProjection;
};

struct VSIn
{
    float3 PosL : POSITION;
    float4 Color : COLOR0;
    float3 Normal : NORMAL0;
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
    
    // I want the diffuse color to be red.
    // I do this by describing that it reflects 100% red light, and completely absorbs green and blue.
    float3 material_diffuse_color = { 1.0, 0.0, 0.0 };

    // The object should reflect 100% of ambient light color
    float3 material_ambient_color = { 0.15, 0.15, 0.15 };

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
    float3 diffuse_calculation = light_diffuse_color * material_diffuse_color;
    float3 ambient_calculation = light_ambient_color * material_ambient_color;

    float3 lit_color = ambient_calculation + float3(
        diffuse_calculation.x * lamberts_multiplier, 
        diffuse_calculation.y * lamberts_multiplier, 
        diffuse_calculation.z * lamberts_multiplier);

    output.Color = float4(lit_color, 1.0);

    return output;
}