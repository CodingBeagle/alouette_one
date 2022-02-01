# Matrices

When doing graphics programming, there are 3 important aspects to consider for matrices:

- How do you store your matrices in memory?
  - Row-Major vs. Column-Major Layout?
- What coordinate system does your matrices base themselves on?
  - Right-Handed Vs Left-Handed
- What convention do you use Row and Column Vectors?
  - Are they a column in the matrix? Or a row at the bottom row of the matrix?

# Row-Major vs. Column-Major as Memory Storage

When talking about matrices in **computing**, one important topic is the choice of how to interpret the memory layout of a matrix. As in, how are the elements of a matrix laid out in memory.

- Row-Major
  - Elements are laid out sequentially in memory, left-to-right, from top to bottom row.
- Column-Major
  - Elements are laid out sequentially in memory, top-to-bottom, left to right. That is, you scan each column, left to right, top to bottom.

# Row-Major vs. Column-Major Vector Convention

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

# Left-Handed vs Right-Handed (or any other interpretation of orientation)

