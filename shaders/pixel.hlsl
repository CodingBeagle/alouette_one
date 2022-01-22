// Pixel shader

struct PSIn
{
    float4 PosH : SV_POSITION;
    float4 Color : COLOR0;
};

float4 PS(PSIn input) : SV_TARGET
{
    return input.Color;
}
