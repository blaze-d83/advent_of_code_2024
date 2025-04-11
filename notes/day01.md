# Day 01: [Historian Hysteria]

## Problem Description

### In a nutshell: 

- Part 1: Find the sum of differences between the same index values of two sorted arrays.
- Part 2: Find the similarity score of repititive values in List A and B.

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

This time, you'll need to figure out exactly how often each number from the left list appears in the right list. Calculate a total similarity score by adding up each number in the left list after multiplying it by the number of times that number appears in the right list.

Here are the same example lists again:

    3   4
    4   3
    2   5
    1   3
    3   9
    3   3

For these example lists, here is the process of finding the similarity score:

- The first number in the left list is 3. It appears in the right list three times, so the similarity score increases by 3 * 3 = 9.
- The second number in the left list is 4. It appears in the right list once, so the similarity score increases by 4 * 1 = 4.
- The third number in the left list is 2. It does not appear in the right list, so the similarity score does not increase (2 * 0 = 0).
- The fourth number, 1, also does not appear in the right list.
- The fifth number, 3, appears in the right list three times; the similarity score increases by 9.
- The last number, 3, appears in the right list three times; the similarity score again increases by 9.

So, for these example lists, the similarity score at the end of this process is 31 (9 + 4 + 0 + 0 + 9 + 9).

Once again consider your left and right lists. What is their similarity score?

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

### Part Two
1. Use the exsting fucntion to parse the values
2. Use the exisiting function to do the string conversion
3. Store all unique values from List A in another array
4. Check number of times each value is repeated and add product of value by number of times it is repeated in List B to return value.
5. Return the sum of all.
---

## TODO
- [x] Write a function that reads the input.txt file and returns two arrays of type string
- [x] Write a function that takes in string type array and converts it to i32 type array
- [x] Write the final function that iterates the two arrays and sums up the differences between values at each index and returns the sum
- [x] Call these functions in `run()`
- [ ] Write a function to store all unque values from List A in an array
- [ ] Wite a function to conduct the similarity score


---

## Example Cases
| Input                   | Expected Output Part 1 | Expected Output Part 2 |
|--------------------------|------------------------|-------------------------|
|                          |                        |                         |
