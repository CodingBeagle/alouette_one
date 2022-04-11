# Shading Models

In computer graphics, a shading model is the technique used to describe how an object's color should vary based on different factors, such as its surface orientation, the view direction, and lighting.

The process of shading is done on the GPU, by programs called **shaders**.

When we use lightning, vertex colors are not longer specified directly - such as simple examples where you provide direct RGB values for each vertex manually.

Instead, materials and light is specified (basically data describing how a surface should react to light, and the type of light we want to use), which is then used in a **lighting equation**, which computes the vertex colors for us.

The term **"unlit"** is typically used to mean rendered vertex colors that have not been applied by a ligthing equation, but are rather just direct RGB color values.

## Real World Light and Color

According to the **trichromatic theory**, the eye has three kinds of light receptors. Each receptor is respectively sensitive to red, green, and blue light.

Depending on how much each receptor is stimulated, you will experience surfaces as being red, green, blue, or a mixture of them all.

Surfaces (or materials) in the real world can each be seen as absorbing varying degrees of red, green, and blue light. The color of the surface as you see it depends on how much it absords of each RGB value, and thus how much of each RGB value is reflected back into your eye.

Not only that, but surfaces in the real world are also each affected by the light bouncing off of other surfaces around it.

## Global and Local Illumination Models

In shading, we typically differentiate between two ways of modelling lighting.

A *local illumination model* simply means that each object is lit independently of other objects. The only thing being taken into account for each surface is the light as it is received directly from the various light sources in the scene (so, light bouncing off of other objects is ignored). A consequence of this is that things like shadows are not actually a naturally occuring concept that the lighting equations will model. If you have a light source in front of a wall, and then a sphere behind that wall, the sphere will still be colored as receiving light.

A *global illumination model* takes into consideration not only the light emitted from the light sources, but also the indirect light that has bounced off other objects in the scene. The models are called global illumination models because they take everything in the global scene into consideration when lighting an object.

## Face Normals and Surface Normals

**Face Normal** = A unit vector that describes the direction a polygon is facing. Typically that polygon will be a triangle, and so the face normal is a unit vector that is orthogonal to the plane on which all the points of the triangle lies.

**Surface Normal** = A unit vector that is orthogonal to the tangent plane of a point on a surface. Surface normals can be imagined as describing the direction a point on the surface is "facing", outwardly away from the surface.

**Vertex Normals** = Surface normals for the vertex points making up the model.

To calculate light, we typically need the vertex normals, so the "direction" that the vertices are pointing away from the surface. We use this to figure out how direct of a hit the light is on the surface making up the vertices.<>

## Typical Vectors used in Light Equations

- Vertex Normal
  - Some sort of vector that indicates the direction of the surface. This may be simple directions such as the directions from each vertex, or it can be more advanced like the interpolated surface normal at each pixel point.
- Light Vector
  - The vector from the surface to the light source. This is simple a vector that points in the direction from the surface to the light source (so, opposite direction of what the light travels).

## Phong Shading

Phong Shading is a specifc shading technique in which you interpolate a surface normal from the surrounding vertex normals of a triangle being rasterized, and then from that interpolated surface normal, a color value is calculated on a *per pixel basis*. Thus, phong shading color is usually calculated in the *pixel shader*.

## Flat Shading

In contrast to *Phong Shading*, *Flat Shading* is a less computationally expensive way of calculating colors for rasterized pixels.

In flat shading, you apply the lighting calculations per vertex. This result is then an output from the vertex shader, and the resulting color of each vertex is then used to interpolate a color accross the pixels of entire triangles.
