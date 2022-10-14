This programming assignment was worked on by Adrian Sicaju Ruiz and Andre Quiroa. 
We had a recieevd alot of help through the TA Connor Gray and classmates from the discord server. 
Essentially what we had implemented was a a function that took a collection of pixels in a two dimensional
array. We were able to create a function that supports images, a two-dimensional array 
(a collection of an image's pixels),and a vector abstraction with support of a polymorphic two-dimensional array called Array2. 
The Array2 vector abstraction contained properties such as width and height to measure and identify an element's position.
We used iterators to visit elements in order and functions such as a row-major and column-major to visit every element of the first 
row in order by column and column-major to visit every element of the first leftmost column followed by every element of 
the second column. We were able to test these using sudoku and essentially if a solved sudoku graymap image was given, 
we would receieved an exit 0. If not, it would receive an exit 1 to alert that the sudoku puzzle is not a solved graymap.
This assignment took us approximately over 25-30 hours to complete at best with consideration of an extension. 