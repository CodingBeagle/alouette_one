# Resources

Resources in DirectX 11 are areas of memory that can be accessed by the Direct3D pipeline.

Resources typically contain the following types of data:

- Geometry
- Textures
- Shader Data

Typical way of working with resources:

1. Create a resource.
2. Bind a resource to the pipeline using a device context.
3. Deallocate a resource by calling Release of the resource interface.
