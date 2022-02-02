# Quaternions in Graphics Programming

Quaternions can be used to rotate vectors in 3D space.

A quaternion in graphics programming is usually described as containing a **scalar** component and a **3D vector component**.

*q = [w (x y z)]*

## Advantages and Disadvantages of Quaternions

Advantages:

- **Smooth Interpolation**
  - The interpolation provided by the *slerp* and *squad* operations provides smooth interpolation between orientations.
- **Fast concatenation and inversion of angular displacements**
  - We can concatenate a sequence of angular displacements into a single angular displacement using the quaternion cross product (multiplication).
- **Fast conversion to/from matrix form**
  - Quaternions can be converted to and from matrix form slighty faster than Euler Angles.
- **Only Four Numbers**
  - Quaternions are considerably more economical than a matrix, because a quaternion only uses 4 numbers, whilst the matrix will use 9.

## Properties of Quaternions

### Quaternion Negation

For quaternions, each angular displacement in 3D has exactly **two** distinct representations in quaternion format. They are each other's negatives.

### Identity Quaternion

The identity quaternion, that is a quaternion which when multiplied by a quaternion **q**, results in **q**, equals:

*q = [1 0 0 0]*

Multiplying any quaternion by this quaternion will not change it.

## Quaternions as an Axis-Angle Pair

It's common to interpret a quaternion as an axis-angle representation of angular displacement about an axis.

That is, you have an angle *A* in radians which will be rotated about a specific Axis **n**.

The *direction* of **n** defines which way is considered "positive" rotation, according to the *handedness* of the coordinate system.

The way that the angel and the axis of rotation is stored as a quaternion is as follows:

*q = [ w x y z ] = [ cos(A/2) (sin(A/2)nx sin(A/2)ny sin(A/2)nz)]*
