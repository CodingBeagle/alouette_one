DXBC2 ¶ЧOйт§¬ќо+»   ьg     8   М  ь  p  X	  ф	  RDEFL     h      <    ю€  $  RD11<          (   $          \                             cbPerObject \      А   ј           p      @      Р      €€€€    €€€€    і  @   @      Р      €€€€    €€€€    ј  А         Ў      €€€€    €€€€    ь  Р         Ў      €€€€    €€€€    	  †         Ў      €€€€    €€€€      ∞         Ў      €€€€    €€€€    worldViewProjection float4x4 ЂЂЂ                            Д  modelMatrix cameraPosition float4 ЂЂ                            ѕ  diffuseColor ambientColor specularColor Microsoft (R) HLSL Shader Compiler 10.1 ISGNh         P                    Y                    _                   POSITION COLOR NORMAL ЂЂOSGNl         P                    \                    b                   SV_POSITION COLOR NORMAL ЂЂЂSHEXа  P  x  jИ Y  FО         _  r     _  r    g  т         e  т     h     6  r      F     6  В      @    А?       F     FО            "     F     FО           B     F     FО           В     F     FО         6  r     @        А?        6  r     @    А?  А?  А?    6  r     @    А?  А?  А?           F    F    4       
     @      8  в     	    Й      	   8  r     F    FВ      
   6  r     @    А?  А?  А?    6  r     FВ                F     FО           "     F     FО           B     F     FО         6  r      FАA         r      F     F      В      F     F     D  В      :      8  r      ц     F       
В      @        Ањ        F       В      :      :      6  В      : АA       8  r     ц     F       
r     F    @        Ањ        8  r     F    FВ                 F    F     4        
      @      6  "      @    А?8  "      
            8        
      
      8        
      
      8        
              "      @      
            6        @        8       
          8  "     
     *     8  B     
     :        в      	    	    8       
      
     8  "     
           8  B     
      *        r      Ц     F    6  r      F     6  В      @    А?6  т      F    6  т     F     >  STATФ   5             %                                                                                                                                SPDB ^  Microsoft C/C++ MSF 7.00
DS         /   а       -                                                                                                                                                                                                                                                                                                                                                                                                                                                                           ј€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€8    ј€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€€       <       €€€€                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         Ф.1eUmb   уьА≠¶!aIЩНЎyRЈ”µ                          №Q3                                                                                                                                                                                                                                                                                                                                                                                                                                                                    entColor;
    float4 specularColor;
};

struct VSIn
{
    float3 PosL : POSITION;
    float4 Color : COLOR;
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
    // The verteLи ∆Z  хѕ ’л  &ч Нн … 1ы ®—  gЯ ќ7  9ќ йр Aє                                                                                                                                                                                                                                                                                                                                                                                                                                                                        // a "cbuffer" is a Constant Buffer.
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
    float4 Color : COLOR;
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
}                                                                                                                                                                                                                                                                                                                юпюп   ?   D:\personal-projects\alouette_one\shaders\vertex.hlsl  d:\personal-projects\alouette_one\shaders\vertex.hlsl // a "cbuffer" is a Constant Buffer.
// Constant buffers are blocks of memory which can store variables which can be 
// Accessed by a shader.
// Data is constant buffers doesn't vary per vertex, but stays the same.
cbuffer cbPerObject : register(b0)
{
    float4x4 worldViewProjection;
    float4x4 modelMatrix;
    float4 cameraPosition;
    float4 diffuseColor;
    float4 ambiв0А   Њ€ѕ¶\Ў                                                               8   (   в0Сќ"ƒ–     7   8                                                                                                                                                                                                                                                                                                                                                                                                                  B <   
   ЇG
   ЇGMicrosoft (R) HLSL Shader Compiler 10.1   2 =hlslFlags 0x5 hlslTarget vs_5_0 hlslEntry VS   *     Ў
      И      И    X    †VS   . >  	 input                                  P     X    И     P    X    И    P    X    И    P    X    И    P    X    И    P    X    И    P    X    И    P    X    И     P     X    И$    P  $  X    И(   : >  И <VS return value>                                  P     X    И     P  $  X    И$    P  (  X    И(    P    X    И    P    X    И    P    X    И    P    X    И    P     X    И     P    X    И    P    X    И    P    X    И   . >   output                                 P      †    @    P     ј         P     а         P         а    P     †   @      P     †   @     P     †   @     P     і   ,    : >    sun_light_direction                                P          \      P         Ш $    P         Ш (   : >    light_diffuse_color                                P      @   Ш 0    P     @   Ш 4    P     @   Ш 8   : >    light_ambient_color                                P      `   Ш @    P     `   Ш D    P     `   Ш H   : >@     lamberts_multiplier                                P      Ш   †    : >    diffuse_calculation                                P      Є   Ь$    P     Є   Є(    P     Є   (,   : >    ambient_calculation                                P      Ў   0    P     Ў   4    P     Ў   8   : >    specular_light_color                               P      ш   Є@    P     ш   ЄD    P     ш   ЄH   B >    camera_position_truncated                                  P         XP    P        XT    P        XX   J >   surface_point_in_world_coordinates                                 P      0   ∞`    P     P   Рd    P     p   ph   2 >    view_vector                                P      р   №      P     р       P     р   ,   : >    reflection_vector                                  P      Р   8P    P     Р   TT    P     Р   pX   : >    specular_calculation                               P      ∞   0@    P     ∞   0D    P     ∞   0H   6 >@     specular_factor                                P      l        2 >    lit_color                                  P      М   T      P     М   T     P     М   T      ф         КЖ»K4€ш∞ъTч’bЯ  т           а      j     X   %  АX   %   l   %  Аl   %   А   %  АА   %   †   %  А†   %   ј   %  Ај   %   а   %  Аа   %      ,  А   ,      3  А   3   @  6  А@  6   `  <  А`  <   |  <  А|  <   Ш  =  АШ  =   Є  >  АЄ  >   Ў  A  АЎ  A   ш  K  Аш  K     L  А  L   0  L  А0  L   P  L  АP  L   p  N  Аp  N   И  N  АИ  N   §  N  А§  N   ј  N  Ај  N   ‘  N  А‘  N   р  O  Ар  O     O  А  O   4  O  А4  O   L  O  АL  O   h  O  Аh  O   Р  P  АР  P   ∞  Q  А∞  Q   ћ  Q  Аћ  Q   и  Q  Аи  Q   ь  Q  Аь  Q     Q  А  Q   4  Q  А4  Q   P  Q  АP  Q   l  S  Аl  S   И  S  АИ  S   Ф  U  АФ  U   ®  V  А®  V   ђ  Z  Йђ  \   »  Z  Й»  ]   д  Z  Йд  ^      Z  Й   Z    Z  Й  `   8  Z  Й8  a   T  Z  ЙT  b   p  Z  Йp  Z  	М  e  АМ  e   †  e  А†  e   і  g  Аі  g   »  g  А»  g   №  g  А№  g    E  D  E  D  E  D  E  D  E  D  E  D  6  5  3  2  3  2  R % J  R ! Q  H " G  H " G  8  7  d  c  [ 1 Z  [ 1 Z  [ 1 Z  g $ e  g $ e  g  f  g  f  g  f  P   O  P   O  P   O  P   O  P   O  K # J  e % G  e ! N  e  d  e  d  e  d  e  d  e  d  $ 	 #  $  $ 	  	       
  7  
  7  
  7  
  8  
  4  
  4  
  4  
  	  *  )  *  )             ц                    <   \   А   §   »                                                                                                                                                                                                                                                                                                                                                                                                                                                                    18        P  
 €€   €€     8   8      @        @       float3 утс @       float4 утс6       PosL с    Color      Normal утс              ( VSIn с
      6      PosH с    Color       Normal утс              , VSOut 
             @             @ float4x4 
 	    
 
    
     
                                                                                                                              18              €€   €€                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 x shader just does the projection matrix.
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
}    8   7               n                                                                                                                                                          D3DSHDR а                             `                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        €€€€	/с8      …                  •      ]      Б      =                 @                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             $   0   <   H                                                                                                                                                                                                                                                                                                                                                                                                                            %    |    VS    & Q       €€€€€€worldViewProjection  Q     @ €€€€€€modelMatrix " Q     А €€€€€€cameraPosition  " Q     Р €€€€€€diffuseColor    " Q     † €€€€€€ambientColor    " Q     ∞ €€€€€€specularColor                                                                                                                                                                                                                                                                                                                  €€€€	/с                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            Ф.1eUmb   уьА≠¶!aIЩНЎyRЈ”µc   /LinkInfo /names /src/headerblock /src/files/d:\personal-projects\alouette_one\shaders\vertex.hlsl                       "      
                 №Q3                                                                                                                                                                                                                                                                                                                                 €€€€w	1     О ?\   H       ,   D                                    а     `             	 №
      D                VS none -Ї.с       а     `                    €€€€    а        €€€€    €€€€         D:\personal-projects\alouette_one\shaders\vertex.hlsl   юпюп                  €€€€€€€€€€ €€€€€€€€€€                                                                                                                                                                                                њ   И  G  8       o  А   –  @  @       (   h  ,   м      *      +                      !   "   #   $         	   
                                                   %   &   '   )   (                                                                                                                                                                                                                                                                                                   ,                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               