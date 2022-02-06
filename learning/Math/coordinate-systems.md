# Coordinate Systems

# Cartesian Coordinate Space

## Screen Coordinate Space

For screen coordinate space, it is customary to use a coordinate system where the origin is in the upper left-hand corner, +x pointing right, and +y pointing **down**.

## 2D Cartesian Space

- Has a special location called the **origin**, which is the "center" of the coordinate system.
- Has two straight lines that pass through the origin. Each line is known as an **axis** and extends infinitely in two opposite directions. The two aces are perpendicular to each other.

## 3D Cartesian Space

3D cartesian space has three axes, all perpendicular to each other.

The axes are customarily called: X, Y, Z

There are no real standard for which of the axes goes "where", and whether positive numbers on an axis goes right, or up, or down and left. That entirely depends on the authors you read and the software you use.

Also, **not all 3D coordinate spaces are equal**, in the sense that some 3D coordinate spaces cannot be rotated in any manner to "line up", in the sense that all axes have the same direction.

For example, you can have a 3D coordinate system in which the positive Z-axis points "into" the screen. You can also have a 3D coordinate system in which the positive Z-axis points "out of" the screen instead, that is, towards the viewer.

No matter how you try to rotate or mirror one of the coordinate systems, you will always end up with one axis pointing in the wrong direction.

From this comes the concept of **left-handed** and **right-handed** 3D coordinate spaces. Two coordinate spaces have the same **handedness** if they can be rotated such that their axes align in direction. If they are opposite handedness, then this will not be possible.

The most common left-hand vs right-hand coordinate system you might see discussed in computer graphics is the ones where positive Z goes into the screen or out of the screen. Converting one from the other in this case is easy. Not in terms of rotation (as this would be impossible), but in terms of simply flipping the sign of the Z coordinate.

## Handedness in 3D Graphics Design

When writing software for 3D graphics, it's important to adopt a decision on:

- Which handedness you want to work with, which direction is +y, +x, +z, etc. And then just stick with it. This is important to decide upon and remember throughout the process as you might need to adopt certain equations and other calculations found in resources to suit the handedness you've chosen, versus what other authors might have chosen. And usually, it will only be a matter of flipping signs.