# Input Assembler Stage

The input-assembler stage reads primitive data (points, lines, triangles) from user filled buffers and assemble the data into primitives that will be used by the other pipeline stages.

## Initializing the Input-Assembler

To initialize the input-assembler (IA) stage, you need to:

1. Create buffer resources with vertex data that the pipeline needs.
   1. Such as vertex buffers, index buffers...
2. Tell the IA stage where the buffers are, and what type of data they hold.
3. Specify the type of primitives to assemble from the data.

## Input-Layout Object

The Input-Layout Object encapsulates the input state of the IA stage.

It describes the input data that is bound to the IA stage.

The data is streamed into the IA stage from memory, from one or more vertex buffers.

