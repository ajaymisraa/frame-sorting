# Frame-Sorting (rust-based)
Frame sorting algorithm optimized inspired by VSCO frame sorting. Couldn't find a rust-based use case for this so I developed an algorithm that should work well. 

This is intended for photos but there are loads of applications for this to consider this as a frame or as a box or rectangle, etc. 

## Overview
The primary goal is to optimize the search for the best position (pos_x, pos_y) to place a new photo on a canvas, considering the photo's width and ensuring it does not overlap with already placed photos. The optimization focuses on minimizing the vertical space (height) used on the canvas by placing photos in the shortest available column.

### Min-heap for Columns
The columns_heap acts as a min-heap based on column heights, enabling quick access to the shortest column, which is the most likely candidate for the next photo placement.

### Dynamic Position Search
Instead of iterating over every possible position, the find_position_for_photo function leverages the min-heap to find a suitable x position where the photo can fit without exceeding the canvas width and not overlapping taller columns.

## Benefits
R-Tree is the "next best thing" for this. The use of a min-heap significantly reduces the complexity of finding the shortest column from O(n) — what R-tree uses — to O(log n) — what this uses —, where n is the number of columns. This approach scales better with the number of photos and canvas size, as it avoids exhaustive searches and efficiently manages the dynamic placement of photos. From what I could tell, VSCO and Instagram uses an R-Tree approach, similarly, Python approaches use a hash-map sequencing alogrithm, which, too, uses O(n) efficiency- putting a slight strain on loading times. 
