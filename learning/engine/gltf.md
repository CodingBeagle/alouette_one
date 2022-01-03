# glTF

## Nodes

The "nodes" property of a glTF asset contains the objects that makes up a scene.

## Accessors

Accessors describe the number and the format of data elements stored in a binary buffer, more specifically by describing the elements of an associated buffer view.

## Buffers

Buffers in a gltf asset is arbitrary data stored as a binary blob. This blob can be a combination of geometry, animation, skins, images, etc.

## Buffer Views

A buffer view represents a contiguous segment of data in a buffer, described by the properties "byteLength" and "byteOffset".

- byteLength = The length of the segment in the buffer.
- byteOffset = The byte offset in the buffer, where the segment begins.