# Day 01: [Historian Hysteria]

## Problem Description

In a nutshell: 
        Find the sum of differences between the same index values of two sorted arrays

In detail:

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
1. Parse the input.
2. Implement solution for part 1:
   - Steps:
3. Solve part 2 (if revealed):
   - Steps:
4. Write tests for edge cases and example cases.

---

## TODO
- [ ] Parse input.
- [ ] Write the `part1` function.
- [ ] Test `part1` with example data.
- [ ] Solve part 2.
- [ ] Optimize if needed.

---

## Notes
- (Additional thoughts about the problem.)

---

## Example Cases
| Input                   | Expected Output Part 1 | Expected Output Part 2 |
|--------------------------|------------------------|-------------------------|
|                          |                        |                         |
