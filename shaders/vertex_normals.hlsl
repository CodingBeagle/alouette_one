cbuffer cbPerObject : register(b0)
{
    float4x4 worldViewProjection;
};

struct VSIn
{
    float3 PosL : POSITION;
};

struct VSOut
{
    float4 PosH : SV_POSITION;
    float4 Color : COLOR0;
};

VSOut VS(VSIn input)
{
    VSOut output;
    output.PosH = mul(float4(input.PosL.x, input.PosL.y, input.PosL.z, 1.0f), worldViewProjection);
    output.Color = float4(0.0, 0.0, 0.0, 1.0);
    return output;
}