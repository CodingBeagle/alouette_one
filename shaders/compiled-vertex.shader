DXBC??kf?`/4!?.?Hz   ?g     8   ?  ?  P  8	  ?	  RDEFL     h      <    ??  $  RD11<          (   $          \                             cbPerObject \      ?   ?           p      @      ?      ????    ????    ?  @   @      ?      ????    ????    ?  ?         ?      ????    ????    ?  ?         ?      ????    ????    	  ?         ?      ????    ????      ?         ?      ????    ????    worldViewProjection float4x4 ???                            ?  modelMatrix cameraPosition float4 ??                            ?  diffuseColor ambientColor specularColor Microsoft (R) HLSL Shader Compiler 10.1 ISGNH         8                    A                   POSITION NORMAL OSGNl         P                    \                    b                   SV_POSITION COLOR NORMAL ???SHEX?  P  x  j? Y  F?         _  r     _  r    g  ?         e  ?     h     6  r      F     6  ?      @    ??       F     F?            "     F     F?           B     F     F?           ?     F     F?         6  r     @         ?   ?    6  r     @    ??  ??  ??    6  r     @    ??  ??  ??           F    F    4       
     @      8  ?     	    ?      	   8  r     F    F?      
   6  r     @    ??  ??  ??    6  r     F?                F     F?           "     F     F?           B     F     F?         6  r      F?A         r      F     F      ?      F     F     D  ?      :      8  r      ?     F       
?      @        ??        F       ?      :      :      6  ?      : ?A       8  r     ?     F       
r     F    @        ??        8  r     F    F?                 F    F     4        
      @      6  "      @    ??8  "      
            8        
      
      8        
      
      8        
              "      @      
            6        @        8       
          8  "     
     *     8  B     
     :        ?      	    	    8       
      
     8  "     
           8  B     
      *        r      ?     F    6  r      F     6  ?      @    ??6  ?      F    6  ?     F     >  STAT?   5             %                                                                                                                                SPDB ^  Microsoft C/C++ MSF 7.00
DS         /   ?       ,                                                                                                                                                                                                                                                                                                                                                                                                                                                                           ????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????8    ???????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????       <       ????                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         ?.1tb   ???P?H??k?0???                          ?Q3                                                                                                                                                                                                                                                                                                                                                                                                                                                                    entColor;
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
    // The vertex shader just does the projL? ?^ ??  ? ?Z  ?? ? 1? ?? g? ?7  9? ? A?                                                                                                                                                                                                                                                                                                                                                                                                                                                                        // a "cbuffer" is a Constant Buffer.
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
    float3 sun_light_direction = { 0.0f, 0.5f, -0.5f };

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
}                                                                                                                                                                                                                                                                                                                                          ????   %   D:\personal-projects\alouette_one\shaders\vertex.hlsl  d:\personal-projects\alouette_one\shaders\vertex.hlsl // a "cbuffer" is a Constant Buffer.
// Constant buffers are blocks of memory which can store variables which can be 
// Accessed by a shader.
// Data is constant buffers doesn't vary per vertex, but stays the same.
cbuffer cbPerObject : register(b0)
{
    float4x4 worldViewProjection;
    float4x4 modelMatrix;
    float4 cameraPosition;
    float4 diffuseColor;
    float4 ambi?0?   ؂?ͧ`?                                                               8   (   ?0-?|?     7   8                                                                                                                                                                                                                                                                                                                                                                                                                  B <   
   ?G
   ?GMicrosoft (R) HLSL Shader Compiler 10.1   2 =hlslFlags 0x5 hlslTarget vs_5_0 hlslEntry VS   *     x
      ?      ?    X    ?VS   . >  	 input                                  P     X    ?     P    X    ?    P    X    ?    P    X    ?    P    X    ?    P    X    ?   : >  ? <VS return value>                                  P     X    ?     P  $  X    ?$    P  (  X    ?(    P    X    ?    P    X    ?    P    X    ?    P    X    ?    P     X    ?     P    X    ?    P    X    ?    P    X    ?   . >   output                                 P      ?    @    P     ?         P     ?         P         ?    P     ?   @      P     ?   @     P     ?   @     P     ?   ,    : >    sun_light_direction                                P          \      P         ? $    P         ? (   : >    light_diffuse_color                                P      @   ? 0    P     @   ? 4    P     @   ? 8   : >    light_ambient_color                                P      `   ? @    P     `   ? D    P     `   ? H   : >@     lamberts_multiplier                                P      ?   ?    : >    diffuse_calculation                                P      ?   ?$    P     ?   ?(    P     ?   (,   : >    ambient_calculation                                P      ?   0    P     ?   4    P     ?   8   : >    specular_light_color                               P      ?   ?@    P     ?   ?D    P     ?   ?H   B >    camera_position_truncated                                  P         XP    P        XT    P        XX   J >   surface_point_in_world_coordinates                                 P      0   ?`    P     P   ?d    P     p   ph   2 >    view_vector                                P      ?   ?      P     ?       P     ?   ,   : >    reflection_vector                                  P      ?   8P    P     ?   TT    P     ?   pX   : >    specular_calculation                               P      ?   0@    P     ?   0D    P     ?   0H   6 >@     specular_factor                                P      l        2 >    lit_color                                  P      ?   T      P     ?   T     P     ?   T      ?         ????B?!?s]'?P  ?           ?      j     X   $  ?X   $   l   $  ?l   $   ?   $  ??   $   ?   $  ??   $   ?   $  ??   $   ?   $  ??   $      +  ?   +      2  ?   2   @  5  ?@  5   `  ;  ?`  ;   |  ;  ?|  ;   ?  <  ??  <   ?  =  ??  =   ?  @  ??  @   ?  J  ??  J     K  ?  K   0  K  ?0  K   P  K  ?P  K   p  M  ?p  M   ?  M  ??  M   ?  M  ??  M   ?  M  ??  M   ?  M  ??  M   ?  N  ??  N     N  ?  N   4  N  ?4  N   L  N  ?L  N   h  N  ?h  N   ?  O  ??  O   ?  P  ??  P   ?  P  ??  P   ?  P  ??  P   ?  P  ??  P     P  ?  P   4  P  ?4  P   P  P  ?P  P   l  R  ?l  R   ?  R  ??  R   ?  T  ??  T   ?  U  ??  U   ?  Y  ??  [   ?  Y  ??  \   ?  Y  ??  ]      Y  ?   Y    Y  ?  _   8  Y  ?8  `   T  Y  ?T  a   p  Y  ?p  Y  	?  d  ??  d   ?  d  ??  d   ?  f  ??  f   ?  f  ??  f   ?  f  ??  f    E  D  E  D  E  D  E  D  E  D  E  D  7  6  3  2  3  2  R % J  R ! Q  H " G  H " G  8  7  d  c  [ 1 Z  [ 1 Z  [ 1 Z  g $ e  g $ e  g  f  g  f  g  f  P   O  P   O  P   O  P   O  P   O  K # J  e % G  e ! N  e  d  e  d  e  d  e  d  e  d  $ 	 #  $  $ 	  	       
  7  
  7  
  7  
  8  
  4  
  4  
  4  
  	  *  )  *  )             ?                    <   \   ?   ?   ?                                   ?18        @  
 ??   ??     8   8      @        @       float3 ???&       PosL ?     Normal ???               VSIn ?
       @       float4 ???6      PosH ?    Color       Normal ???              , VSOut 
             @             @ float4x4 
 	    
 
    
     
                                                                                                                                             ?18              ??   ??                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 ection matrix.
    // A vector on the LHS (PosL in this case) will be treated as a ROW VECTOR by HLSL.
    VSOut output;
    output.PosH = mul(float4(input.PosL, 1.0f), worldViewProjection);

    // ** Color is calculated using lightning equation **

    // Hardcoded sun light direction in world coordinates
    // Currently a noon-sun (directly overhead)
    // Notice that the light vector is opposite of where the light rays are actually coming from, which would be -1 on the Y axis.
    float3 sun_light_direction = { 0.0f, 0.5f, -0.5f };

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
}    8   7   n                                                                                                                                                                                                D3DSHDR ?                             `                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        ????	/?8      ?                  ?      ]      ?      =                 @                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             $   0   <   H                                                                                                                                                                                                                                                                                                                                                                                                                            %    |    VS    & Q       ??????worldViewProjection  Q     @ ??????modelMatrix " Q     ? ??????cameraPosition  " Q     ? ??????diffuseColor    " Q     ? ??????ambientColor    " Q     ? ??????specularColor                                                                                                                                                                                                                                                                                                                  ????	/?                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            ?.1tb   ???P?H??k?0???c   /LinkInfo /names /src/headerblock /src/files/d:\personal-projects\alouette_one\shaders\vertex.hlsl                       "      
                 ?Q3                                                                                                                                                                                                                                                                                                                                 ????w	1     ? ?\   H       ,   D                                    ?     `             	 |
      D   -             VS none -?.?       ?     `                    ????    ?        ????    ????         D:\personal-projects\alouette_one\shaders\vertex.hlsl   ????                  ?????????? ??????????                                                                                                                                                                                                ?   x  G  8       U  ?   ?  ?  @       (   h  ,   ?      )      *                         !   "   #         	   
                                                $   %   &   (   '                                                                                                                                                                                                                                                                                                       +                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               