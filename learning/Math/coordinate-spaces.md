# Coordinate Spaces

That something has its "own" coordinate space simply means that its origin and axes, and direction of its axes, it defined all relative to itself.

## World Space

The world coordinate system establishes the "largest" coordinate system. In that, you cannot express the world coordinate in terms of anything "outside" of it, or something encompassing it. The world coordinate system is absolute.

Questions typically asked in world space:

- What is the position and orientation of each object?
- What is the position and orientation of the camera?
- Who does each object get from where it is to where it wants to be? (For example, path-finding).

It can also be helpful to think of the **world space** as the **root** of a tree with child coordinate spaces, which again can have their own child coordinate spaces. Every coordinate space will be defined in terms of a parent to which it is relative, except the world space, which is the root.

## Object Space

Object Space, also sometimes called **model space**, is a coordinate space associated with a particular object.

Every object has its own independent coordinate space. And some concepts only make sense in the context of an object space.

For example, **directional** concepts such as: forward, backward, left, right, only make sense in object coordinate space. In terms of an "outer" world coordinate system, such a concept couldn't exist.

**Positions** can also be relative to a local object space. For example, the position of vertices of a model will be relative to its own coordinate space, with its own origin and directions.

## Camera Space

Camera space is the coordinate space associated with an observer.
