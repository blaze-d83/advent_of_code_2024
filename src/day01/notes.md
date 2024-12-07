# Day 01: [Historian Hysteria]

## Problem Description

### In a nutshell: 
        Part 1: Find the sum of differences between the same index values of two sorted arrays

### In detail:

#### Part One

        3   4
        4   3
        2   5
        1   3
        3   9
        3   3

To find out, pair up the numbers and measure how far apart they are. Pair up the smallest number in the left list with the smallest number in the right list, then the second-smallest left number with the second-smallest right number, and so on.

Within each pair, figure out how far apart the two numbers are; you'll need to add up all of those distances. For example, if you pair up a 3 from the left list with a 7 from the right list, the distance apart is 4; if you pair up a 9 with a 3, the distance apart is 6.

To find the total distance between the left list and the right list, add up the distances between all of the pairs you found. In the example above, this is 2 + 1 + 0 + 1 + 2 + 5, a total distance of 11!

Your actual left and right lists contain many location IDs. What is the total distance between your lists?

#### Part Two

---

## Brainstorming

### Input structure:
```text
    15131   78158
    32438   35057
    12503   57702
    73808   43128
    57168   71761
```


###  Key observations:
1. Input data is in `.txt` format. So, parsing it, I will have to store the first value on each line in one array / vector, and the other in another.
2. The data is unsorted, so I will have to implement sorting on both the arrays.
3. I will have to figure out how am I iterating both the arrays, comparing the same index values, and saving the differences / summing it up as we iterate. If we save in another array, will have to walk that array and sum up the values once more (not ideal).

###  Edge cases:
- I can't think of edge cases at any step, i mean we iterate, store, sort, and iterate again to get our result. Highly unlikely to run into data overflow, which we will most likely catch during compilation (rust btw).

### Language specific considerations:
- Since it is rust (btw), I will have to make sure that we borrow any value once.
- So if I try to image what the lifetime of values should like it, it will be:
    Input data -> (parse) -> unsorted array A & B -> (sorting algo) -> sorted array A & B -> intialize mutable `sum` variable -> w:q



---

## Plan

### Part one
1. Parse the input.txt file:
    - Store first value on each line as a string in one array, and the next value as a string in another array.
2. String Conversion
    - Do string conversion of both arrays
3. Sort
    - Use the inbuilt sort function
4. Initialize a common mutable value.
5. Iterate both the arrays and store the sum of differences between each indexed values to the mutable value.
6. Return the mutable value as the result.

---

## TODO
- [x] Write a function that reads the input.txt file and returns two arrays of type string
- [x] Write a function that takes in string type array and converts it to i32 type array
- [x] Write the final function that iterates the two arrays and sums up the differences between values at each index and returns the sum
- [x] Call these functions in `run()`

---

## Notes
- (Additional thoughts about the problem.)

---

## Example Cases
| Input                   | Expected Output Part 1 | Expected Output Part 2 |
|--------------------------|------------------------|-------------------------|
|                          |                        |                         |
