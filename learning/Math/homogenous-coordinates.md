# Homogenous Coordinates

Homogenous coordinates are (typically, in the case of graphics programming), a 4D vector with the last component being referred to as **w**.

## Usefulness of Homogenous Coordinates

All of the linear transformations:
- Scaling
- Rotation
- Shearing
- Reflection

Can be represented using a 3x3 matrix, with each row (if you use row vectors) representing the basis vectors of a coordinate system.

However, a 3x3 matrix cannot represent translation.

However, a 4x4 matrix can be used to embed translation in the bottom row. Since it's a 4x4 matrix, we also need to multiply it by a 4D vector (remember that a vector can also be thought of a matrix with a single row). The last element of the 4D vector, **w**, can be set to either **0** or **1**, indicating a vector or a point.
- 0 = vector.
- 1 = point.

Pragmatically, setting the **w** element of a vector to **0** switches off translation. You'd get the same result as only multiplying that vector by a 3x3 matrix containing a linear transform (or a combination of linear transforms).