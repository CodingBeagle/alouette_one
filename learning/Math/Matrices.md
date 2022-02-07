# Matrices

When doing graphics programming, there are 3 important aspects to consider for matrices:

- How do you store your matrices in memory?
  - Row-Major vs. Column-Major Layout?
- What coordinate system does your matrices base themselves on?
  - Right-Handed Vs Left-Handed
- What convention do you use Row and Column Vectors?
  - Are they a column in the matrix? Or a row at the bottom row of the matrix?

## Row-Major vs. Column-Major as Memory Storage

When talking about matrices in **computing**, one important topic is the choice of how to interpret the memory layout of a matrix. As in, how are the elements of a matrix laid out in memory.

- Row-Major
  - Elements are laid out sequentially in memory, left-to-right, from top to bottom row.
- Column-Major
  - Elements are laid out sequentially in memory, top-to-bottom, left to right. That is, you scan each column, left to right, top to bottom.

## Row-Major vs. Column-Major Vector Convention

A confusing aspect of matrix math when talking transforms in graphics programming, is that there is also different conventions used for how vectors are represented in matrices. That is, are they represent as rows in the matrix, or columns?

- Row-Major Vector Convention
  - Vectors in the transform matrices are rows.
  - Translation vectors are stored in the bottom row of the matrix.
  - Rows of the matrix represent the bases (x, y, z) of the coordinate system.
  - When using Row-Major convention, vectors should be pre-multiplied to the matrix (that is, vectors are the left-hand side.)
  - Transforms of rotation and translation are opposite order of Column-Major.
- Column-Major Vector Convention
  - Vectors in the transform matrices are columns.
  - Translation vectors are stored in the outmost right column of the matrix.
  - Columns of the matrix represent the bases (x, y, z) of the coordinate system.
  - When using Column-Major convention, vectors should be post-multiplied to the matrix (that is, vectors are the right-hand side.)
  - Transforms of rotation and translation are opposite order of Row-Major.

## Geometric Interpretation

General: A square matrix can describe any **linear transformation**.

Useful sets of linear transformations commonly used:

- Rotation
- Scale
- Orthographic Projection
- Reflection
- Shearing

**Key Idea** about matrices is this:

All a matrix really represents is a **coordinate space transformation**, taking a vector being expressed in (x, y, z) coordinates, and changing it into the same vector, just expressed in a different coordinate space.

Thus, for the expression: **aM=b**

Where *a* and *b* are vectors, and *M* a matrix, it is said that M transformed **a** into **b**.

**And this is all that really happens, fundamentally, with matrices in graphics programming**. No Matter if you're talking about translating, scaling, rotating, etc... all you're ever doing is transforming points from one representation into another.

## Transforming objects vs coordinate spaces

When you transform something, you can either transform an object, or the coordinate space of the object.

It's two different things.

When you transform an object, you are transforming the **points** of the object. The points move to a new position in the *same* coordinate space that was always used. So the points will have new coordinates.

When you transform a coordinate space, the points don't actually move. Instead, their location is just expressed using a different coordinate space.

You can essentially get the same results using each method, but each method have advantages and disadvantages, depending on the situation.

## Rotation in 3D

For reference, rotation in 2D occours about a point.

In probably the simplest case, it would be rotation about the origin of the 2D coordinate system. In this case, the only input is the *angle* about which to rotate a point about the origin.

In 3D, however, rotation occurs around an **axis**. A line which something rotates about. It doesn't have to be cardinal (x, y, z), it can be any arbritrary 3D line.

### The definition of positive rotation

Positive rotation in 3D is defined depending on what handedness your coordinate system is.