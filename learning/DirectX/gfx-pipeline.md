# Graphics Pipeline

## Projection

A projection matrix, whether it be *orthographic* or *perspective*, **transforms a volume in a unit cube.**.

A perspective matrix transforms what can be visualized as a *view frustum* into a unit cube.

Process of vertex coordinates:

1. Vertex enters Vertex Shader in 3D coordinate (x, y, z).
2. Vertex is made into homogeneous coordinate by adding "1" as the w component.
   1. (x, y, z, 1)
3. Vertex is multiplied by the perspective transform to get *homogeneous clip space*.
4. Clipping is performed
5. Perspective divide is performed
   1. x, y, z is divided by the w-component to normalized the coordinates.
6. A vertex is applied the perspective transform.
7. Following by clipping.
8. Followed by the perspective divide (dividing the vertex by w).
9. Resutling in normalized device coordinates.

Projection -> Clip Coordinates -> Clipping -> Perspective Divide -> Normalized Device Coordinates.

## Vertex Shader

A vertex shader must always end up producing a coordinate in homogenous coordinates (x, y, z, w) in order for the following phases to work correctly, such as clipping.

## Rasterizer Stage

The rasterizer stage assumes homogenous clip-space coordinates as input.

It is in the rasterizer stage which will:

- Perform clipping to the view frustum.
- Perform the perspective divide.
  - The perspective divide gives you **normalized device coordinates.**
  - In DirectX the **NDC** z coordinates will usually range from 0 to 1.
- Maps primitives to a 2D viewport using **Viewports**.
