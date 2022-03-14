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
    output.Color = input.Color;

    return output;
}