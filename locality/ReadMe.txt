Locality
Memembers: Andre Q, Adrian R
Our major component for this program is our two-dimensional arrays from Array2 that will
support a column-major and row-major that will map out the image’s pixels with column-major
and row-major storage.
To do this, we will use the row-major and col-major to copy pixels from the source image using
iter_row_major and iter_col_major. Row-major refers to visiting every element of the first row in
order by column followed by every element of the second row and column-major visiting every
element of the first left-most column followed by every element of the second column.
Using the data from row-major and column-major, our ppmtrans program will have functions
named rotate90, rotate180, rotate270, flipHoriz, and flipVert, to rotate an image at 0-90-180-270
degrees and flip it horizontal or vertical, or transpose. This image will be read from standard
input or from a file named on the command line of the terminal. The calculation to determine a
90 degrees image are the pixels(I , j) in where the original becomes pixel (h – j - 1, i) in the
rotated image and when its 180 degrees, the pixels(I ,j) becomes pixel(w – I – 1 ,h – j -1)
In the process of our image rotations, we will also create a function that will estimate the
expected hit rate and performance of each type of rotation. We will use Rust built in tools such
as hyperfine to be able to calculate descriptive statistics and the differences in performances
that are determined by the amount of time spent in closures called by the iterators.
We plan to do more research on the calculations other than the required 90–180-degree
rotations to receive the extra credit. What we will begin doing first is to start writing our ppmtrans
program and the essential functions that will be required to utilize the data from the reused
Array2 library that we built in the last assignment. But during testing we chose to use Professors Array2 due to better functionality with what we wanted to do. We will begin to write our benchmarking functions to be able to describe the measures and performance in locality and see how they can be improved
based on our statistics and assumptions. Each component may take us a couple hours to
complete since we need to be able to grasp the concept of cache and locality performance at a
higher level for better understanding. We should hopefully have our ppmtrans functions written
by Monday and our potential changes in Array2 done by Tuesday and then sort out the
benchmarking and computational aspect of the assignment by Thursday