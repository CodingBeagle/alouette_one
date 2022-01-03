// a "cbuffer" is a Constant Buffer.
// Constant buffers are blocks of memory which can store variables which can be 
// Accessed by a shader.
// Data is constant buffers doesn't vary per vertex, but stays the same.
cbuffer cbPerObject : register(b0)
{
    float4x4 worldViewProjection;
};

void VS(float3 PosL : POSITION, out float4 PosH : SV_POSITION)
{
    // Transform to homogenous clip space
    // Notice that the vertex shader, or any other shader, doesn't do the perspective divide.
    // The perspective divide is done by hardware at a later stage.
    // The vertex shader just does the projection matrix.
    // A vector on the LHS (PosL in this case) will be treated as a ROW VECTOR by HLSL.
    PosH = mul(float4(PosL, 1.0f), worldViewProjection);
}