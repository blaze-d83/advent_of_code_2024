# Project Structure

On a higher level:
- solutions are contained in '/solutions' dir
- solutions are divided by days, so each day as a file 'dayXX' which contains part 1 and part 2 
- each file has a `read_input_file()` function that reads the appropriate input file of the day from the `inputs/` dir
- each solution i.e. either part 1 or part 2 runs isolated by calling the `read_input_file()` and its solution function
- main() reads the flags and pattern matches the flag to the appropriate day and part within the solutions


